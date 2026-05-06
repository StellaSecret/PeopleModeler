// ── PEOPLE MODELER — Person Page JS ──────────────────────

let currentPerson = null;

// ── INIT ──────────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', () => {
  currentPerson = JSON.parse(JSON.stringify(Storage.getCurrent())); // deep clone
  renderAll();
  setupTabs();
  updateOceanInterpretation();
});

function renderAll() {
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
  document.getElementById('profileRole').textContent =
    [p.role, p.context].filter(Boolean).join(' · ');

  const tagsEl = document.getElementById('profileTags');
  tagsEl.innerHTML = (p.tags || []).map(t => `<span class="tag">${t}</span>`).join('');

  // Update accuracy ring
  const score = p.accuracyScore || 0;
  const circumference = 2 * Math.PI * 34; // r=34
  const offset = circumference - (score / 100) * circumference;
  const circle = document.querySelector('.accuracy-ring circle:last-child');
  if (circle) circle.style.strokeDashoffset = offset;
  const label = document.querySelector('.accuracy-label');
  if (label) label.innerHTML = `${score}%<br/><small>précision</small>`;
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

  if (motivations.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;">Aucune motivation ajoutée.</p>`;
    return;
  }

  list.innerHTML = motivations.map((m, i) => {
    const def = MOTIVATIONS.find(x => x.id === m.type) || { emoji: '?', label: m.type };
    return `
      <div class="motivation-item" data-index="${i}">
        <div class="mot-icon">${def.emoji}</div>
        <div class="mot-info">
          <div class="mot-name">${def.label}</div>
          <div class="mot-bar-wrap">
            <div class="mot-bar"><div class="mot-bar-fill" style="width:${m.intensity * 10}%"></div></div>
            <div class="mot-intensity">${m.intensity}/10</div>
          </div>
          ${m.notes ? `<div class="mot-notes">"${m.notes}"</div>` : ''}
        </div>
        <button class="btn-delete" onclick="deleteMotivation(${i})" title="Supprimer">✕</button>
      </div>`;
  }).join('');
}

function openAddMotivation() {
  document.getElementById('modalTitle').textContent = '💡 Ajouter une motivation';
  document.getElementById('modalContent').innerHTML = `
    <label>Type de motivation</label>
    <select id="motType">
      ${MOTIVATIONS.map(m => `<option value="${m.id}">${m.emoji} ${m.label}</option>`).join('')}
    </select>
    <label>Intensité : <span id="motIntensityVal">5</span>/10</label>
    <input type="range" min="1" max="10" value="5" id="motIntensity"
           oninput="document.getElementById('motIntensityVal').textContent=this.value" />
    <label>Notes (optionnel)</label>
    <input type="text" id="motNotes" placeholder="Comportement observé…"
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

function deleteMotivation(index) {
  currentPerson.motivations.splice(index, 1);
  savePerson();
  renderMotivations();
}

// ── BIASES ────────────────────────────────────────────────
function renderBiases() {
  const list = document.getElementById('biasList');
  const biases = currentPerson.biases || [];

  if (biases.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;">Aucun biais ajouté.</p>`;
    return;
  }

  list.innerHTML = biases.map((b, i) => {
    const def = BIASES.find(x => x.id === b.type) || { emoji: '?', label: b.type };
    return `
      <div class="bias-item" data-index="${i}">
        <div class="mot-icon">${def.emoji}</div>
        <div class="mot-info">
          <div class="mot-name">${def.label}</div>
          <div class="mot-bar-wrap">
            <div class="mot-bar"><div class="bias-bar-fill" style="width:${b.intensity * 10}%"></div></div>
            <div class="mot-intensity">${b.intensity}/10</div>
          </div>
          ${b.evidence ? `<div class="mot-notes">"${b.evidence}"</div>` : ''}
        </div>
        <button class="btn-delete" onclick="deleteBias(${i})" title="Supprimer">✕</button>
      </div>`;
  }).join('');
}

function openAddBias() {
  document.getElementById('modalTitle').textContent = '🧠 Ajouter un biais cognitif';
  document.getElementById('modalContent').innerHTML = `
    <label>Type de biais</label>
    <select id="biasType">
      ${BIASES.map(b => `<option value="${b.id}">${b.emoji} ${b.label}</option>`).join('')}
    </select>
    <label>Intensité observée : <span id="biasIntensityVal">5</span>/10</label>
    <input type="range" min="1" max="10" value="5" id="biasIntensity"
           oninput="document.getElementById('biasIntensityVal').textContent=this.value" />
    <label>Preuve / exemple observé (optionnel)</label>
    <input type="text" id="biasEvidence" placeholder="Situation concrète observée…"
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
  if (!el || !currentPerson.ocean) return;
  const o = currentPerson.ocean;
  const name = currentPerson.name.split(' ')[0];

  const desc = key => {
    const val = o[key];
    const d = OCEAN_DESCRIPTIONS[key];
    return val >= 6 ? d.high : d.low;
  };

  el.innerHTML = `
    <strong>${name}</strong> est ${desc('O')}.
    ${o.C >= 6 ? `Consciencieux·se, ${desc('C')}.` : `${desc('C').charAt(0).toUpperCase() + desc('C').slice(1)}.`}
    ${o.E >= 6 ? `Très ${desc('E')}.` : `Plutôt ${desc('E')}.`}
    En termes relationnels, ${name} est ${desc('A')}.
    Sur le plan émotionnel : ${desc('N')}.
    <br/><br/>
    <strong>À retenir :</strong>
    ${o.E >= 7 && o.A <= 4 ? `⚡ Profil "dominant" — direct, assertif, peut sembler intimidant.` : ''}
    ${o.O >= 8 && o.C >= 7 ? `🚀 Profil innovateur rigoureux — rare et précieux.` : ''}
    ${o.N >= 7 ? `⚠️ Attention en situation de stress — réactivité émotionnelle élevée.` : ''}
    ${o.A >= 8 && o.N <= 4 ? `🤝 Profil stable et coopératif — excellent médiateur.` : ''}
  `;
}

// ── PREDICTIONS ───────────────────────────────────────────
function renderPredictions() {
  const list = document.getElementById('predictionList');
  const preds = currentPerson.predictions || [];

  if (preds.length === 0) {
    list.innerHTML = `<p style="color:var(--text-muted);font-size:.88rem;margin-bottom:1rem;">Aucune prédiction enregistrée.</p>`;
    return;
  }

  list.innerHTML = preds.map((p, i) => `
    <div class="prediction-item">
      <div class="pred-context">📍 ${p.context}</div>
      <div class="pred-outcome">🔮 ${p.predictedOutcome}</div>
      <div class="pred-status">
        <span class="pred-badge ${p.resolved ? 'resolved' : 'pending'}">
          ${p.resolved ? '✅ Résolue' : '⏳ En attente'}
        </span>
        ${p.resolved && p.accuracy ? `<span style="color:var(--gold);font-size:.78rem;">Précision : ${p.accuracy}/10</span>` : ''}
        ${p.resolved && p.actualOutcome ? `<span style="color:var(--text-muted);font-size:.78rem;">→ ${p.actualOutcome}</span>` : ''}
        ${!p.resolved ? `<button class="btn-resolve" onclick="openResolvePrediction(${i})">Résoudre →</button>` : ''}
      </div>
    </div>
  `).join('');
}

function addPrediction() {
  const context = document.getElementById('predContext').value.trim();
  const outcome = document.getElementById('predOutcome').value.trim();
  if (!context || !outcome) { alert('Remplissez le contexte et la prédiction.'); return; }

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
  document.getElementById('modalTitle').textContent = '✅ Résoudre la prédiction';
  const pred = currentPerson.predictions[index];
  document.getElementById('modalContent').innerHTML = `
    <p style="color:var(--text-muted);font-size:.85rem;margin-bottom:1rem;">
      Prédiction : <em>"${pred.predictedOutcome}"</em>
    </p>
    <label>Ce qui s'est réellement passé</label>
    <input type="text" id="resolveActual" placeholder="Résultat réel…"
           style="width:100%;background:var(--bg);border:1px solid var(--border);border-radius:var(--radius);padding:.75rem 1rem;color:var(--text);font-family:var(--font-mono);font-size:.88rem;outline:none;margin:.5rem 0 1rem;"/>
    <label>Précision de la prédiction : <span id="resolveAccVal">7</span>/10</label>
    <input type="range" min="1" max="10" value="7" id="resolveAcc"
           oninput="document.getElementById('resolveAccVal').textContent=this.value" />
  `;
  window._modalConfirm = () => {
    const actual = document.getElementById('resolveActual').value.trim();
    const accuracy = parseInt(document.getElementById('resolveAcc').value);
    if (!actual) { alert('Décrivez ce qui s\'est passé.'); return; }
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
    label: 'sous stress',
    generate(p) {
      const topMot = getTopMotivation(p);
      const topBias = getTopBias(p);
      const lines = [`Sous stress, ${p.name.split(' ')[0]} aura tendance à :`];
      if (p.ocean?.N >= 7) lines.push('• Réagir émotionnellement et perdre de la recul');
      if (p.ocean?.E >= 7) lines.push('• Exprimer verbalement son stress, chercher à en parler');
      if (p.ocean?.E <= 4) lines.push('• Se replier sur soi, éviter les interactions');
      if (p.ocean?.C >= 7) lines.push('• Sur-contrôler, micro-manager, demander des updates constants');
      if (topMot?.type === 'POWER') lines.push('• Reprendre le contrôle par l\'autorité');
      if (topMot?.type === 'SECURITY') lines.push('• Chercher des garanties et certitudes');
      if (topBias) lines.push(`• Être particulièrement sujet au biais "${BIASES.find(b=>b.id===topBias.type)?.label}"`);
      lines.push('\n💡 Stratégie : ' + (p.ocean?.A >= 6 ? 'Offrez du soutien émotionnel avant les solutions.' : 'Proposez des actions concrètes et rapides.'));
      return lines.join('\n');
    }
  },
  conflict: {
    label: 'en conflit',
    generate(p) {
      const lines = [`En situation de conflit, ${p.name.split(' ')[0]} :`];
      if (p.ocean?.A <= 4) lines.push('• N\'hésite pas à s\'opposer frontalement');
      if (p.ocean?.A >= 7) lines.push('• Cherche à éviter l\'affrontement, cherche le compromis');
      if (p.ocean?.N >= 7) lines.push('• Peut prendre les choses personnellement');
      if (p.ocean?.E >= 7) lines.push('• Exprime le conflit ouvertement, ne laisse pas traîner');
      const topMot = getTopMotivation(p);
      if (topMot?.type === 'POWER') lines.push('• Cherche à "gagner" le conflit plutôt qu\'à le résoudre');
      if (topMot?.type === 'AFFILIATION') lines.push('• Souffre du conflit relationnel, veut préserver le lien');
      const lossBias = p.biases?.find(b => b.type === 'LOSS_AVERSION');
      if (lossBias?.intensity >= 6) lines.push('• Fort biais de perte : refusera de "lâcher" même si rationnel');
      lines.push('\n💡 Approche : ' + (p.ocean?.A >= 6 ? 'Cadrez comme "résolution commune", pas opposition.' : 'Soyez direct·e et factuel·le, évitez les ambiguïtés.'));
      return lines.join('\n');
    }
  },
  success: {
    label: 'en réussite',
    generate(p) {
      const lines = [`En période de réussite, ${p.name.split(' ')[0]} :`];
      const recMot = p.motivations?.find(m => m.type === 'RECOGNITION');
      if (recMot?.intensity >= 7) lines.push('• A besoin que le succès soit reconnu publiquement');
      const powMot = p.motivations?.find(m => m.type === 'POWER');
      if (powMot?.intensity >= 7) lines.push('• Va capitaliser sur le succès pour renforcer son influence');
      if (p.ocean?.O >= 7) lines.push('• Cherche déjà le prochain défi ou projet ambitieux');
      if (p.ocean?.C >= 7) lines.push('• Analyse ce qui a fonctionné pour le répliquer');
      const dkBias = p.biases?.find(b => b.type === 'DUNNING_KRUGER');
      if (dkBias?.intensity >= 6) lines.push('⚠️ Risque de surconfiance après le succès');
      lines.push('\n💡 C\'est le bon moment pour proposer de nouveaux projets ou renforcer la relation.');
      return lines.join('\n');
    }
  },
  uncertainty: {
    label: 'dans l\'incertitude',
    generate(p) {
      const lines = [`Face à l\'incertitude, ${p.name.split(' ')[0]} :`];
      if (p.ocean?.N >= 7) lines.push('• Génère de l\'anxiété, peut paralyser la décision');
      if (p.ocean?.N <= 3) lines.push('• Reste remarquablement calme, peut sous-estimer les risques');
      if (p.ocean?.O >= 7) lines.push('• Voit l\'incertitude comme une opportunité de créativité');
      if (p.ocean?.O <= 4) lines.push('• Très inconfortable, cherche à revenir à des routines connues');
      const secMot = p.motivations?.find(m => m.type === 'SECURITY');
      if (secMot?.intensity >= 7) lines.push('• Forte anxiété — cherche des certitudes à tout prix');
      const ancBias = p.biases?.find(b => b.type === 'ANCHORING');
      if (ancBias?.intensity >= 6) lines.push('• S\'accroche aux dernières données connues comme ancre');
      lines.push('\n💡 Réduisez l\'incertitude perçue : donnez un maximum de contexte et de repères.');
      return lines.join('\n');
    }
  },
  recognition: {
    label: 'cherchant la reconnaissance',
    generate(p) {
      const recMot = p.motivations?.find(m => m.type === 'RECOGNITION');
      const lines = [`Quand ${p.name.split(' ')[0]} cherche de la reconnaissance :`];
      if (recMot) {
        const intensity = recMot.intensity;
        if (intensity >= 8) lines.push('• Besoin intense — sera démotivé·e si ignoré·e longtemps');
        else if (intensity >= 5) lines.push('• Besoin modéré — apprécie la reconnaissance sans en dépendre');
        else lines.push('• Peu motivé·e par la reconnaissance externe');
      }
      if (p.ocean?.E >= 7) lines.push('• Préfère la reconnaissance publique, devant l\'équipe');
      if (p.ocean?.E <= 4) lines.push('• Préfère une reconnaissance privée et sincère');
      const spBias = p.biases?.find(b => b.type === 'SOCIAL_PROOF');
      if (spBias?.intensity >= 6) lines.push('• Très sensible à l\'opinion des pairs et au statut');
      lines.push('\n💡 Nommez explicitement la contribution. La reconnaissance vague est contre-productive.');
      return lines.join('\n');
    }
  },
  threat: {
    label: 'se sentant menacé·e',
    generate(p) {
      const lines = [`Quand ${p.name.split(' ')[0]} se sent menacé·e :`];
      const powMot = p.motivations?.find(m => m.type === 'POWER');
      if (powMot?.intensity >= 7) lines.push('• Réaction dominante : attaque ou contre-offensive');
      if (p.ocean?.A <= 4) lines.push('• Peut devenir défensif·ve et agressif·ve');
      if (p.ocean?.A >= 7) lines.push('• Cherche d\'abord à désamorcer, peut sur-accommoder');
      if (p.ocean?.N >= 7) lines.push('• Rumination, peut lire des menaces là où il n\'y en a pas');
      const laBias = p.biases?.find(b => b.type === 'LOSS_AVERSION');
      if (laBias?.intensity >= 6) lines.push('• L\'aversion aux pertes amplifie la réaction : "je ne peux pas perdre ça"');
      const confBias = p.biases?.find(b => b.type === 'CONFIRMATION');
      if (confBias?.intensity >= 6) lines.push('• Cherche des preuves qui confirment la menace, ignore le reste');
      lines.push('\n💡 Approche : rassurez sur ce qui n\'est PAS menacé avant d\'aborder le problème.');
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
  event.target.classList.add('active');

  const template = INSIGHT_TEMPLATES[triggerKey];
  if (!template || !currentPerson) return;

  const output = document.getElementById('insightOutput');
  output.innerHTML = `<div style="color:var(--text-muted);font-size:.78rem;margin-bottom:.75rem;">
    Analyse comportementale — contexte : <strong style="color:var(--cyan)">${template.label}</strong>
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
