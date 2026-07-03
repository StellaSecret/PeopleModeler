// ── WASM Bridge ────────────────────────────────────────────
// Loads Rust core via WASM, patches global data/functions.
// Falls back to hardcoded JS if WASM fails.

(function() {
  const MOT_EMOJIS = { POWER:'👑', ACHIEVEMENT:'🏆', AFFILIATION:'🤝', SECURITY:'🛡️', AUTONOMY:'🦅', RECOGNITION:'⭐', LEARNING:'📚', HELPING:'❤️' };
  const BIAS_EMOJIS = { CONFIRMATION:'🔄', ANCHORING:'⚓', AVAILABILITY:'📱', SUNK_COST:'💸', DUNNING_KRUGER:'🎭', LOSS_AVERSION:'😰', SOCIAL_PROOF:'👥', AUTHORITY:'🎖️', RECENCY:'⏰', IN_GROUP:'🏠' };

  window.__wasm = null;
  let wasmError = null;
  let patchQueued = true;

  function patchGlobals(m) {
    if (!m) return;
    const lang = (typeof getLang === 'function') ? getLang() : 'fr';

    if (typeof MOTIVATIONS !== 'undefined') {
      MOTIVATIONS.forEach(item => {
        try { item.label = m.mot_label(item.id, lang); } catch(e) {}
        try { item.desc = m.mot_desc(item.id, lang); } catch(e) {}
      });
    }
    if (typeof BIASES !== 'undefined') {
      BIASES.forEach(item => {
        try { item.label = m.bias_label(item.id, lang); } catch(e) {}
        try { item.desc = m.bias_desc(item.id, lang); } catch(e) {}
      });
    }
  }

  function patchOceanFn(m) {
    if (!m || typeof updateOceanInterpretation !== 'function') return;
    const orig = updateOceanInterpretation;
    if (orig.__wasmPatched) return;
    const patched = function() {
      const el = document.getElementById('oceanInterpretation');
      if (!el || !currentPerson || !currentPerson.ocean) { return orig.call(this); }
      try {
        const json = JSON.stringify(currentPerson.ocean);
        const result = m.analyze_ocean(json);
        el.innerHTML = result;
      } catch(e) {
        orig.call(this);
      }
    };
    patched.__wasmPatched = true;
    window.updateOceanInterpretation = patched;
  }

  function patchInsightFn(m) {
    if (!m || typeof showInsight !== 'function') return;
    const orig = showInsight;
    if (orig.__wasmPatched) return;
    const ctxMap = { stress:'decision', conflict:'conflict', success:'success', uncertainty:'uncertainty', recognition:'recognition', threat:'threat' };
    const patched = function(triggerKey) {
      document.querySelectorAll('.trigger-btn').forEach(b => b.classList.remove('active'));
      if (event && event.target) event.target.classList.add('active');

      const ctx = ctxMap[triggerKey] || triggerKey;
      if (!currentPerson) return orig.call(this, triggerKey);
      try {
        const personJson = JSON.stringify(currentPerson);
        const result = m.generate_insight(ctx, personJson);
        const output = document.getElementById('insightOutput');
        const headerKey = `insight_${triggerKey}_header`;
        const header = (typeof t === 'function' && t(headerKey) !== headerKey) ? t(headerKey) : triggerKey;
        output.innerHTML = `<div style="color:var(--text-muted);font-size:.78rem;margin-bottom:.75rem;">${typeof t === 'function' ? t('insight_context_label') : 'Contexte'} : <strong style="color:var(--cyan)">${header}</strong></div>` + result.replace(/\n/g, '<br/>');
      } catch(e) {
        orig.call(this, triggerKey);
      }
    };
    patched.__wasmPatched = true;
    window.showInsight = patched;
  }

  function patchInsightTemplates(m) {
    if (!m || typeof INSIGHT_TEMPLATES === 'undefined') return;
    if (INSIGHT_TEMPLATES.__wasmPatched) return;
    const ctxMap = { stress:'decision', conflict:'conflict', success:'success', uncertainty:'uncertainty', recognition:'recognition', threat:'threat' };
    Object.keys(INSIGHT_TEMPLATES).forEach(key => {
      const ctx = ctxMap[key] || key;
      const origGen = INSIGHT_TEMPLATES[key].generate;
      INSIGHT_TEMPLATES[key].generate = function(p) {
        try {
          const personJson = JSON.stringify(p);
          return m.generate_insight(ctx, personJson);
        } catch(e) {
          return origGen.call(this, p);
        }
      };
    });
    INSIGHT_TEMPLATES.__wasmPatched = true;
  }

  function patchPredictionFn(m) {
    if (!m) return;

    if (typeof addPrediction === 'function' && !addPrediction.__wasmPatched) {
      const orig = addPrediction;
      const patched = function() {
        const ctxEl = document.getElementById('predContext') || document.getElementById('predCtx');
        const outEl = document.getElementById('predOutcome') || document.getElementById('predOut');
        const context = ctxEl?.value?.trim();
        const outcome = outEl?.value?.trim();
        if (!context && !outcome) { orig.call(this); return; }

        const person = typeof currentPerson !== 'undefined' ? currentPerson : window.currentPerson;
        if (!person) { orig.call(this); return; }

        try {
          const personJson = JSON.stringify(person);
          if (typeof m.suggest_prediction === 'function' && context && !outcome) {
            const s = m.suggest_prediction(personJson, context);
            if (s && outEl) outEl.value = s;
          }
          if (typeof m.create_prediction === 'function' && context && outEl?.value?.trim()) {
            const finalOutcome = outEl.value.trim();
            const result = JSON.parse(m.create_prediction(person.id, context, finalOutcome));
            person.predictions = person.predictions || [];
            person.predictions.unshift(result);
            if (ctxEl) ctxEl.value = '';
            if (outEl) outEl.value = '';
            if (typeof savePerson === 'function') savePerson();
            else if (typeof window.save === 'function') window.save();
            if (typeof renderPredictions === 'function') {
              if (renderPredictions.length > 0) renderPredictions(person);
              else renderPredictions();
            }
            return;
          }
        } catch(e) { /* fallback to original */ }
        orig.call(this);
      };
      patched.__wasmPatched = true;
      window.addPrediction = patched;
    }
  }

  function reRenderIfNeeded() {
    if (typeof renderAll === 'function' && currentPerson) {
      renderAll();
    } else if (typeof renderMotivations === 'function' && currentPerson) {
      renderMotivations();
      renderBiases();
    }
  }

  window.__wasmCreatePrediction = function(personId, context, predictedOutcome) {
    const m = window.__wasm;
    if (!m || typeof m.create_prediction !== 'function') return null;
    try {
      return JSON.parse(m.create_prediction(personId, context, predictedOutcome));
    } catch(e) { return null; }
  };

  window.__wasmResolvePrediction = function(predictionObj, actualOutcome, accuracy) {
    const m = window.__wasm;
    if (!m || typeof m.resolve_prediction !== 'function') return null;
    try {
      const input = JSON.stringify({
        id: predictionObj.id, person_id: predictionObj.person_id || predictionObj.personId,
        context: predictionObj.context, predicted_outcome: predictionObj.predictedOutcome || predictionObj.predicted_outcome,
        actual_outcome: null, accuracy: null, created_at: predictionObj.createdAt || predictionObj.created_at,
        resolved_at: null, resolved: false
      });
      return JSON.parse(m.resolve_prediction(input, actualOutcome, accuracy));
    } catch(e) { return null; }
  };

  // Start WASM load immediately
  const wasmLoad = import('../wasm/peoplemodeler_core.js').then(m => {
    window.__wasm = m;
    patchGlobals(m);
    patchOceanFn(m);
    patchInsightFn(m);
    patchInsightTemplates(m);
    patchPredictionFn(m);
    patchQueued = false;
  }).catch(err => {
    wasmError = err;
    patchQueued = false;
    console.warn('WASM core not available, using JS fallback:', err.message);
  });

  // Retry patching on a timer (in case globals defined later)
  let retries = 0;
  const maxRetries = 20;
  const retryInterval = setInterval(() => {
    if (patchQueued && window.__wasm) {
      const m = window.__wasm;
      patchGlobals(m);
      patchOceanFn(m);
      patchInsightFn(m);
      patchInsightTemplates(m);
      patchPredictionFn(m);
    }
    retries++;
    if (!patchQueued || retries >= maxRetries) clearInterval(retryInterval);
  }, 100);

  // Expose a promise for callers who want to await WASM
  window.__wasmReady = wasmLoad.then(() => true).catch(() => false);
  window.__wasmLoad = () => wasmLoad;
})();
