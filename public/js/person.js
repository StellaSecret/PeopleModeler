// ── PEOPLE MODELER — Person Page JS ──────────────────────

let currentPerson = null;

// ── INIT ──────────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', async () => {
  await (window.__wasmReady || Promise.resolve(true));
  currentPerson = JSON.parse(JSON.stringify(Storage.getCurrent())); // deep clone
  renderAll();
  setupTabs();
  if (window.__wasm) updateOceanInterpretation();
  else updateOceanInterpretation();
});

function renderAll() {
  if (!currentPerson) return;
  renderHeader();
  renderMotivations();
  renderBiases();
  renderOcean();
  renderPredictions();
}

// ── HEADER ────────────────────────────────────────────────
function renderHeader() {
  const p = currentPerson;
  document.getElementById('profileAvatar').textContent = p.avatarEmoji || '🧑';
  document.getElementById('profileName').textContent = p.name;

  const isDemo = p.id === 'demo-001';
  const role = isDemo ? t('demo_role') : p.role;
  const ctx = isDemo ? t('demo_context') : p.context;
  document.getElementById('profileRole').textContent =
    [role, ctx].filter(Boolean).join(' · ');

  const tagsEl = document.getElementById('profileTags');
  tagsEl.innerHTML = (p.tags || []).map(tag => {
    const normalized = tag.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, '');
    const tagKey = isDemo ? `demo_tag_${normalized}` : null;
    return `<span class="tag">${tagKey && t(tagKey) !== tagKey ? t(tagKey) : tag}</span>`;
  }).join('');

  // Update accuracy ring
  const score = p.accuracyScore || 0;
  const circumference = 2 * Math.PI * 34; // r=34
  const offset = circumference - (score / 100) * circumference;
  const circle = document.querySelector('.accuracy-ring circle:last-child');
  if (circle) circle.style.strokeDashoffset = offset;
  const label = document.querySelector('.accuracy-label');
  if (label) label.innerHTML = `${score}%<br/><small>${t('person_accuracy')}</small>`;
}

// ── TABS ──────────────────────────────────────────────────
function setupTabs() {
  document.querySelectorAll('.tab').forEach(tab => {
    tab.addEventListener('click', () => {
      document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      tab.classList.add('active');
      document.getElementById(`tab-${tab.dataset.tab}`)?.classList.add('active');
    });
  });
}

// ── MOTIVATIONS ───────────────────────────────────────────
function renderMotivations() {
  const list = document.getElementById('motivationList');
  const motivations = currentPerson.motivations || [];
  const isDemo = currentPerson.id === 'demo-001';

  if (motivations.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;">${t('mot_empty')}</p>`;
    return;
  }

  list.innerHTML = motivations.map((m, i) => {
    const def = MOTIVATIONS.find(x => x.id === m.type) || { emoji: '?', label: m.type };
    const notes = isDemo ? t('demo_mot_' + m.type.toLowerCase() + '_notes') : m.notes;
    return `
      <div class="motivation-item" data-index="${i}" onclick="openEditMotivation(${i})">
        <div class="mot-icon">${def.emoji}</div>
        <div class="mot-info">
          <div class="mot-name">${def.label}</div>
          <div class="mot-bar-wrap">
            <div class="mot-bar"><div class="mot-bar-fill" style="width:${m.intensity * 10}%"></div></div>
            <div class="mot-intensity">${m.intensity}/10</div>
          </div>
          ${notes ? `<div class="mot-notes">"${notes}"</div>` : ''}
        </div>
        <button type="button" class="btn-delete" onclick="event.stopPropagation();deleteMotivation(${i})" title="${t('mot_delete_label')}">✕</button>
      </div>`;
  }).join('');
}

function openAddMotivation() {
  document.getElementById('modalTitle').textContent = t('mot_dialog_title');
  document.getElementById('modalContent').innerHTML = `
    <label>${t('mot_type_label')}</label>
    <select id="motType" onchange="document.getElementById('motDesc').textContent=(MOTIVATIONS.find(m=>m.id===this.value)||{}).desc||''">
      ${MOTIVATIONS.map(m => `<option value="${m.id}">${m.emoji} ${m.label}</option>`).join('')}
    </select>
    <p id="motDesc" class="type-desc">${MOTIVATIONS[0].desc}</p>
    <label>${t('mot_intensity_label')} : <span id="motIntensityVal">5</span>/10</label>
    <input type="range" min="1" max="10" value="5" id="motIntensity"
           oninput="document.getElementById('motIntensityVal').textContent=this.value" />
    <label>${t('mot_notes_label')}</label>
    <input type="text" id="motNotes" placeholder="${t('mot_notes_placeholder')}"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin-top:.25rem;"/>
  `;
  window._modalConfirm = () => {
    const type = document.getElementById('motType').value;
    const intensity = parseInt(document.getElementById('motIntensity').value);
    const notes = document.getElementById('motNotes').value;
    currentPerson.motivations = currentPerson.motivations || [];
    currentPerson.motivations.push({ type, intensity, notes });
    savePerson();
    renderMotivations();
    closeModal();
  };
  openModal();
}

function openEditMotivation(index) {
  const m = currentPerson.motivations[index];
  if (!m) return;
  const isDemo = currentPerson.id === 'demo-001';
  document.getElementById('modalTitle').textContent = t('mot_edit_title');
  const def = MOTIVATIONS.find(x => x.id === m.type) || { emoji: '?', label: m.type };
  const notes = ((isDemo ? t('demo_mot_' + m.type.toLowerCase() + '_notes') : m.notes) || '').replace(/"/g, '&quot;');
  document.getElementById('modalContent').innerHTML = `
    <label>${t('mot_type_label')}</label>
    <select id="motType" onchange="document.getElementById('motDesc').textContent=(MOTIVATIONS.find(m=>m.id===this.value)||{}).desc||''">
      ${MOTIVATIONS.map(x => `<option value="${x.id}" ${x.id === m.type ? 'selected' : ''}>${x.emoji} ${x.label}</option>`).join('')}
    </select>
    <p id="motDesc" class="type-desc">${def.desc||''}</p>
    <label>${t('mot_intensity_label')} : <span id="motIntensityVal">${m.intensity}</span>/10</label>
    <input type="range" min="1" max="10" value="${m.intensity}" id="motIntensity"
           oninput="document.getElementById('motIntensityVal').textContent=this.value" />
    <label>${t('mot_notes_label')}</label>
    <input type="text" id="motNotes" value="${notes}" placeholder="${t('mot_notes_placeholder')}"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin-top:.25rem;"/>
  `;
  window._modalConfirm = () => {
    const type = document.getElementById('motType').value;
    const intensity = parseInt(document.getElementById('motIntensity').value);
    const notes = document.getElementById('motNotes').value;
    currentPerson.motivations[index] = { type, intensity, notes };
    savePerson();
    renderMotivations();
    closeModal();
  };
  openModal();
}

function deleteMotivation(index) {
  currentPerson.motivations.splice(index, 1);
  savePerson();
  renderMotivations();
}

// ── BIASES ────────────────────────────────────────────────
function renderBiases() {
  const list = document.getElementById('biasList');
  const biases = currentPerson.biases || [];
  const isDemo = currentPerson.id === 'demo-001';

  if (biases.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;">${t('bias_empty')}</p>`;
    return;
  }

  list.innerHTML = biases.map((b, i) => {
    const def = BIASES.find(x => x.id === b.type) || { emoji: '?', label: b.type };
    const evidence = isDemo ? t('demo_bias_' + b.type.toLowerCase() + '_evidence') : b.evidence;
    return `
      <div class="bias-item" data-index="${i}" onclick="openEditBias(${i})">
        <div class="mot-icon">${def.emoji}</div>
        <div class="mot-info">
          <div class="mot-name">${def.label}</div>
          <div class="mot-bar-wrap">
            <div class="mot-bar"><div class="bias-bar-fill" style="width:${b.intensity * 10}%"></div></div>
            <div class="mot-intensity">${b.intensity}/10</div>
          </div>
          ${evidence ? `<div class="mot-notes">"${evidence}"</div>` : ''}
        </div>
        <button type="button" class="btn-delete" onclick="event.stopPropagation();deleteBias(${i})" title="${t('mot_delete_label')}">✕</button>
      </div>`;
  }).join('');
}

function openAddBias() {
  document.getElementById('modalTitle').textContent = t('bias_dialog_title');
  document.getElementById('modalContent').innerHTML = `
    <label>${t('bias_type_label')}</label>
    <select id="biasType" onchange="document.getElementById('biasDesc').textContent=(BIASES.find(b=>b.id===this.value)||{}).desc||''">
      ${BIASES.map(b => `<option value="${b.id}">${b.emoji} ${b.label}</option>`).join('')}
    </select>
    <p id="biasDesc" class="type-desc">${BIASES[0].desc}</p>
    <label>${t('bias_intensity_label')} : <span id="biasIntensityVal">5</span>/10</label>
    <input type="range" min="1" max="10" value="5" id="biasIntensity"
           oninput="document.getElementById('biasIntensityVal').textContent=this.value" />
    <label>${t('bias_evidence_label')}</label>
    <input type="text" id="biasEvidence" placeholder="${t('bias_evidence_placeholder')}"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin-top:.25rem;"/>
  `;
  window._modalConfirm = () => {
    const type = document.getElementById('biasType').value;
    const intensity = parseInt(document.getElementById('biasIntensity').value);
    const evidence = document.getElementById('biasEvidence').value;
    currentPerson.biases = currentPerson.biases || [];
    currentPerson.biases.push({ type, intensity, evidence });
    savePerson();
    renderBiases();
    closeModal();
  };
  openModal();
}

function openEditBias(index) {
  const b = currentPerson.biases[index];
  if (!b) return;
  const isDemo = currentPerson.id === 'demo-001';
  document.getElementById('modalTitle').textContent = t('bias_edit_title');
  const def = BIASES.find(x => x.id === b.type) || { emoji: '?', label: b.type };
  const evidence = ((isDemo ? t('demo_bias_' + b.type.toLowerCase() + '_evidence') : b.evidence) || '').replace(/"/g, '&quot;');
  document.getElementById('modalContent').innerHTML = `
    <label>${t('bias_type_label')}</label>
    <select id="biasType" onchange="document.getElementById('biasDesc').textContent=(BIASES.find(b=>b.id===this.value)||{}).desc||''">
      ${BIASES.map(x => `<option value="${x.id}" ${x.id === b.type ? 'selected' : ''}>${x.emoji} ${x.label}</option>`).join('')}
    </select>
    <p id="biasDesc" class="type-desc">${def.desc||''}</p>
    <label>${t('bias_intensity_label')} : <span id="biasIntensityVal">${b.intensity}</span>/10</label>
    <input type="range" min="1" max="10" value="${b.intensity}" id="biasIntensity"
           oninput="document.getElementById('biasIntensityVal').textContent=this.value" />
    <label>${t('bias_evidence_label')}</label>
    <input type="text" id="biasEvidence" value="${evidence}" placeholder="${t('bias_evidence_placeholder')}"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin-top:.25rem;"/>
  `;
  window._modalConfirm = () => {
    const type = document.getElementById('biasType').value;
    const intensity = parseInt(document.getElementById('biasIntensity').value);
    const evidence = document.getElementById('biasEvidence').value;
    currentPerson.biases[index] = { type, intensity, evidence };
    savePerson();
    renderBiases();
    closeModal();
  };
  openModal();
}

function deleteBias(index) {
  currentPerson.biases.splice(index, 1);
  savePerson();
  renderBiases();
}

// ── OCEAN ─────────────────────────────────────────────────
function renderOcean() {
  const ocean = currentPerson.ocean || { O: 5, C: 5, E: 5, A: 5, N: 5 };
  Object.entries(ocean).forEach(([key, val]) => {
    const slider = document.getElementById(`slider-${key}`);
    const valEl = document.getElementById(`val-${key}`);
    if (slider) slider.value = val;
    if (valEl) valEl.textContent = val;
  });
  updateOceanInterpretation();
}

function updateOcean(key, value) {
  if (!currentPerson.ocean) currentPerson.ocean = { O: 5, C: 5, E: 5, A: 5, N: 5 };
  currentPerson.ocean[key] = parseInt(value);
  document.getElementById(`val-${key}`).textContent = value;
  updateOceanInterpretation();
  savePerson();
}

function updateOceanInterpretation() {
  const el = document.getElementById('oceanInterpretation');
  if (!el || !currentPerson || !currentPerson.ocean) return;

  if (window.__wasm) {
    try {
      const result = window.__wasm.analyze_ocean(JSON.stringify(currentPerson.ocean));
      el.innerHTML = result;
      return;
    } catch(e) { /* fall through to JS */ }
  }

  const o = currentPerson.ocean;
  const name = currentPerson.name.split(' ')[0];

  const desc = key => {
    const val = o[key];
    const L = getLang();
    const dk = `ocean_${key.toLowerCase()}_${val >= 6 ? 'high' : 'low'}`;
    return L[dk] || OCEAN_DESCRIPTIONS[key][val >= 6 ? 'high' : 'low'];
  };

  el.innerHTML = `
    <strong>${name}</strong> ${t('ocean_interp_is')} ${desc('O')}.
    ${o.C >= 6 ? `${t('ocean_interp_consciencieux')}, ${desc('C')}.` : `${desc('C').charAt(0).toUpperCase() + desc('C').slice(1)}.`}
    ${o.E >= 6 ? `${t('ocean_interp_very')} ${desc('E')}.` : `${t('ocean_interp_rather')} ${desc('E')}.`}
    ${t('ocean_interp_relationally')}, ${name} ${t('ocean_interp_is')} ${desc('A')}.
    ${t('ocean_interp_emotionally')} : ${desc('N')}.
    <br/><br/>
    <strong>${t('ocean_interp_remember')}</strong>
    ${o.E >= 7 && o.A <= 4 ? t('ocean_interp_dominant') : ''}
    ${o.O >= 8 && o.C >= 7 ? t('ocean_interp_innovator') : ''}
    ${o.N >= 7 ? t('ocean_interp_stress_warn') : ''}
    ${o.A >= 8 && o.N <= 4 ? t('ocean_interp_stable') : ''}
  `;
}

// ── PREDICTIONS ───────────────────────────────────────────
function renderPredictions() {
  const list = document.getElementById('predictionList');
  const preds = currentPerson.predictions || [];
  const isDemo = currentPerson.id === 'demo-001';

  if (preds.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;margin-bottom:1rem;">${t('pred_empty')}</p>`;
    return;
  }

  list.innerHTML = preds.map((p, i) => {
    const ck = 'demo_pred_' + p.id + '_context';
    const ok = 'demo_pred_' + p.id + '_outcome';
    const ak = 'demo_pred_' + p.id + '_actual';
    const ctx = isDemo && t(ck) !== ck ? t(ck) : p.context;
    const outcome = isDemo && t(ok) !== ok ? t(ok) : p.predictedOutcome;
    const actual = isDemo && t(ak) !== ak ? t(ak) : p.actualOutcome;
    return `
    <div class="prediction-item">
      <div class="pred-context">📍 ${ctx}</div>
      <div class="pred-outcome">🔮 ${outcome}</div>
      <div class="pred-status">
        <span class="pred-badge ${p.resolved ? 'resolved' : 'pending'}">
          ${p.resolved ? t('pred_resolved') : t('pred_pending')}
        </span>
        ${p.resolved && p.accuracy ? `<span style="color:var(--gold);font-size:.78rem;">${t('pred_accuracy')} : ${p.accuracy}/10</span>` : ''}
        ${p.resolved && actual ? `<span style="color:var(--text-muted);font-size:.78rem;">→ ${actual}</span>` : ''}
        ${!p.resolved ? `<button class="btn-resolve" onclick="openResolvePrediction(${i})">${t('pred_resolve_btn')}</button>` : ''}
      </div>
    </div>`;
  }).join('');
}

function addPrediction() {
  const context = document.getElementById('predContext').value.trim();
  const outcome = document.getElementById('predOutcome').value.trim();
  if (!context || !outcome) { alert(t('pred_alert_fill')); return; }

  currentPerson.predictions = currentPerson.predictions || [];
  currentPerson.predictions.unshift({
    id: 'p' + Date.now(),
    context,
    predictedOutcome: outcome,
    actualOutcome: null,
    accuracy: null,
    createdAt: Date.now(),
    resolved: false,
  });

  document.getElementById('predContext').value = '';
  document.getElementById('predOutcome').value = '';
  savePerson();
  renderPredictions();
}

function openResolvePrediction(index) {
  document.getElementById('modalTitle').textContent = t('pred_resolve_title');
  const pred = currentPerson.predictions[index];
  const isDemo = currentPerson.id === 'demo-001';
  const ok = 'demo_pred_' + pred.id + '_outcome';
  const outcome = isDemo && t(ok) !== ok ? t(ok) : pred.predictedOutcome;
  document.getElementById('modalContent').innerHTML = `
    <p style="color:var(--text-muted);font-size:.85rem;margin-bottom:1rem;">
      ${t('pred_insight_context')} : <em>"${outcome}"</em>
    </p>
    <label>${t('pred_resolve_actual_label')}</label>
    <input type="text" id="resolveActual" placeholder="${t('pred_resolve_actual_placeholder')}"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin:.5rem 0 1rem;"/>
    <label>${t('pred_resolve_acc_label')} : <span id="resolveAccVal">7</span>/10</label>
    <input type="range" min="1" max="10" value="7" id="resolveAcc"
           oninput="document.getElementById('resolveAccVal').textContent=this.value" />
  `;
  window._modalConfirm = () => {
    const actual = document.getElementById('resolveActual').value.trim();
    const accuracy = parseInt(document.getElementById('resolveAcc').value);
    if (!actual) { alert(t('pred_alert_describe')); return; }
    currentPerson.predictions[index].actualOutcome = actual;
    currentPerson.predictions[index].accuracy = accuracy;
    currentPerson.predictions[index].resolved = true;
    currentPerson.predictions[index].resolvedAt = Date.now();
    // Recalculate overall accuracy
    const resolved = currentPerson.predictions.filter(p => p.resolved && p.accuracy);
    if (resolved.length > 0) {
      currentPerson.accuracyScore = Math.round(
        resolved.reduce((sum, p) => sum + p.accuracy, 0) / resolved.length * 10
      );
    }
    savePerson();
    renderPredictions();
    renderHeader();
    closeModal();
  };
  openModal();
}

// ── INSIGHTS ──────────────────────────────────────────────
const INSIGHT_TEMPLATES = {
  stress: {
    generate(p) {
      const topMot = getTopMotivation(p);
      const topBias = getTopBias(p);
      const name = p.name.split(' ')[0];
      const lines = [t('insight_stress_header_line').replace('{name}', name)];
      if (p.ocean?.N >= 7) lines.push('• ' + t('insight_stress_bullet_n_high'));
      if (p.ocean?.E >= 7) lines.push('• ' + t('insight_stress_bullet_e_high'));
      if (p.ocean?.E <= 4) lines.push('• ' + t('insight_stress_bullet_e_low'));
      if (p.ocean?.C >= 7) lines.push('• ' + t('insight_stress_bullet_c_high'));
      if (topMot?.type === 'POWER') lines.push('• ' + t('insight_stress_bullet_power'));
      if (topMot?.type === 'SECURITY') lines.push('• ' + t('insight_stress_bullet_security'));
      if (topBias) lines.push('• ' + t('insight_stress_bullet_top_bias').replace('{bias}', BIASES.find(b=>b.id===topBias.type)?.label || ''));
      lines.push('\n💡 ' + (p.ocean?.A >= 6 ? t('insight_stress_strategy_high_a') : t('insight_stress_strategy_low_a')));
      return lines.join('\n');
    }
  },
  conflict: {
    generate(p) {
      const name = p.name.split(' ')[0];
      const lines = [t('insight_conflict_header_line').replace('{name}', name)];
      if (p.ocean?.A <= 4) lines.push('• ' + t('insight_conflict_bullet_a_low'));
      if (p.ocean?.A >= 7) lines.push('• ' + t('insight_conflict_bullet_a_high'));
      if (p.ocean?.N >= 7) lines.push('• ' + t('insight_conflict_bullet_n_high'));
      if (p.ocean?.E >= 7) lines.push('• ' + t('insight_conflict_bullet_e_high'));
      const topMot = getTopMotivation(p);
      if (topMot?.type === 'POWER') lines.push('• ' + t('insight_conflict_bullet_power'));
      if (topMot?.type === 'AFFILIATION') lines.push('• ' + t('insight_conflict_bullet_affiliation'));
      const lossBias = p.biases?.find(b => b.type === 'LOSS_AVERSION');
      if (lossBias?.intensity >= 6) lines.push('• ' + t('insight_conflict_bullet_loss_aversion'));
      lines.push('\n💡 ' + (p.ocean?.A >= 6 ? t('insight_conflict_strategy_high_a') : t('insight_conflict_strategy_low_a')));
      return lines.join('\n');
    }
  },
  success: {
    generate(p) {
      const name = p.name.split(' ')[0];
      const lines = [t('insight_success_header_line').replace('{name}', name)];
      const recMot = p.motivations?.find(m => m.type === 'RECOGNITION');
      if (recMot?.intensity >= 7) lines.push('• ' + t('insight_success_bullet_recognition_high'));
      const powMot = p.motivations?.find(m => m.type === 'POWER');
      if (powMot?.intensity >= 7) lines.push('• ' + t('insight_success_bullet_power_high'));
      if (p.ocean?.O >= 7) lines.push('• ' + t('insight_success_bullet_o_high'));
      if (p.ocean?.C >= 7) lines.push('• ' + t('insight_success_bullet_c_high'));
      const dkBias = p.biases?.find(b => b.type === 'DUNNING_KRUGER');
      if (dkBias?.intensity >= 6) lines.push('• ' + t('insight_success_bullet_dk'));
      lines.push('\n💡 ' + t('insight_success_strategy'));
      return lines.join('\n');
    }
  },
  uncertainty: {
    generate(p) {
      const name = p.name.split(' ')[0];
      const lines = [t('insight_uncertainty_header_line').replace('{name}', name)];
      if (p.ocean?.N >= 7) lines.push('• ' + t('insight_uncertainty_bullet_n_high'));
      if (p.ocean?.N <= 3) lines.push('• ' + t('insight_uncertainty_bullet_n_low'));
      if (p.ocean?.O >= 7) lines.push('• ' + t('insight_uncertainty_bullet_o_high'));
      if (p.ocean?.O <= 4) lines.push('• ' + t('insight_uncertainty_bullet_o_low'));
      const secMot = p.motivations?.find(m => m.type === 'SECURITY');
      if (secMot?.intensity >= 7) lines.push('• ' + t('insight_uncertainty_bullet_security_high'));
      const ancBias = p.biases?.find(b => b.type === 'ANCHORING');
      if (ancBias?.intensity >= 6) lines.push('• ' + t('insight_uncertainty_bullet_anchoring'));
      lines.push('\n💡 ' + t('insight_uncertainty_strategy'));
      return lines.join('\n');
    }
  },
  recognition: {
    generate(p) {
      const recMot = p.motivations?.find(m => m.type === 'RECOGNITION');
      const name = p.name.split(' ')[0];
      const lines = [t('insight_recognition_header_line').replace('{name}', name)];
      if (recMot) {
        const intensity = recMot.intensity;
        if (intensity >= 8) lines.push('• ' + t('insight_recognition_bullet_intensity_high'));
        else if (intensity >= 5) lines.push('• ' + t('insight_recognition_bullet_intensity_mid'));
        else lines.push('• ' + t('insight_recognition_bullet_intensity_low'));
      }
      if (p.ocean?.E >= 7) lines.push('• ' + t('insight_recognition_bullet_e_high'));
      if (p.ocean?.E <= 4) lines.push('• ' + t('insight_recognition_bullet_e_low'));
      const spBias = p.biases?.find(b => b.type === 'SOCIAL_PROOF');
      if (spBias?.intensity >= 6) lines.push('• ' + t('insight_recognition_bullet_social_proof'));
      lines.push('\n💡 ' + t('insight_recognition_strategy'));
      return lines.join('\n');
    }
  },
  threat: {
    generate(p) {
      const name = p.name.split(' ')[0];
      const lines = [t('insight_threat_header_line').replace('{name}', name)];
      const powMot = p.motivations?.find(m => m.type === 'POWER');
      if (powMot?.intensity >= 7) lines.push('• ' + t('insight_threat_bullet_power_high'));
      if (p.ocean?.A <= 4) lines.push('• ' + t('insight_threat_bullet_a_low'));
      if (p.ocean?.A >= 7) lines.push('• ' + t('insight_threat_bullet_a_high'));
      if (p.ocean?.N >= 7) lines.push('• ' + t('insight_threat_bullet_n_high'));
      const laBias = p.biases?.find(b => b.type === 'LOSS_AVERSION');
      if (laBias?.intensity >= 6) lines.push('• ' + t('insight_threat_bullet_loss_aversion'));
      const confBias = p.biases?.find(b => b.type === 'CONFIRMATION');
      if (confBias?.intensity >= 6) lines.push('• ' + t('insight_threat_bullet_confirmation'));
      lines.push('\n💡 ' + t('insight_threat_strategy'));
      return lines.join('\n');
    }
  },
};

function getTopMotivation(p) {
  return (p.motivations || []).reduce((top, m) => (!top || m.intensity > top.intensity) ? m : top, null);
}
function getTopBias(p) {
  return (p.biases || []).reduce((top, b) => (!top || b.intensity > top.intensity) ? b : top, null);
}

function showInsight(triggerKey) {
  document.querySelectorAll('.trigger-btn').forEach(b => b.classList.remove('active'));
  if (event && event.target) event.target.classList.add('active');
  if (!currentPerson) return;

  const output = document.getElementById('insightOutput');
  const headerKey = `insight_${triggerKey}_header`;
  const header = t(headerKey);

  if (window.__wasm) {
    try {
      const ctxMap = { stress:'decision', conflict:'conflict', success:'success', uncertainty:'uncertainty', recognition:'recognition', threat:'threat' };
      const ctx = ctxMap[triggerKey] || triggerKey;
      const result = window.__wasm.generate_insight(ctx, JSON.stringify(currentPerson));
      output.innerHTML = `<div style="color:var(--text-muted);font-size:.78rem;margin-bottom:.75rem;">
        ${t('insight_context_label')} : <strong style="color:var(--cyan)">${header}</strong>
      </div>` + result.replace(/\n/g, '<br/>');
      return;
    } catch(e) { /* fall through to JS */ }
  }

  const template = INSIGHT_TEMPLATES[triggerKey];
  if (!template) return;
  output.innerHTML = `<div style="color:var(--text-muted);font-size:.78rem;margin-bottom:.75rem;">
    ${t('insight_context_label')} : <strong style="color:var(--cyan)">${header}</strong>
  </div>` + template.generate(currentPerson).replace(/\n/g, '<br/>');
}

// ── MODAL ─────────────────────────────────────────────────
function openModal() {
  document.getElementById('modalOverlay').classList.add('open');
}
function closeModal() {
  document.getElementById('modalOverlay').classList.remove('open');
}
function confirmModal() {
  if (window._modalConfirm) window._modalConfirm();
}

// ── SAVE ─────────────────────────────────────────────────
function savePerson() {
  currentPerson.updatedAt = Date.now();
  Storage.saveCurrent(currentPerson);
}

// ── EXTRA STYLES (injected) ───────────────────────────────
const extraStyles = `
  .btn-delete {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; font-size: 0.8rem; padding: 0.25rem 0.5rem;
    border-radius: 4px; transition: color 0.15s;
  }
  .btn-delete:hover { color: var(--primary); }
  .btn-resolve {
    background: rgba(0,212,170,0.1);
    border: 1px solid rgba(0,212,170,0.3);
    border-radius: 8px;
    color: var(--cyan);
    font-family: var(--font-mono);
    font-size: .75rem;
    padding: .25rem .7rem;
    cursor: pointer;
    transition: background .15s;
  }
  .btn-resolve:hover { background: rgba(0,212,170,0.2); }
`;
const styleEl = document.createElement('style');
styleEl.textContent = extraStyles;
document.head.appendChild(styleEl);
