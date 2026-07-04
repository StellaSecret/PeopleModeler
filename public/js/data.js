// ── PEOPLE MODELER — Data & Constants ────────────────────

const MOTIVATIONS = [
  { id: 'POWER',       label: 'Pouvoir',          emoji: '👑', desc: 'Contrôle des décisions, influence et autorité' },
  { id: 'ACHIEVEMENT', label: 'Accomplissement',   emoji: '🏆', desc: 'Atteinte d\'objectifs ambitieux et performance' },
  { id: 'AFFILIATION', label: 'Appartenance',      emoji: '🤝', desc: 'Relations harmonieuses et appartenance au groupe' },
  { id: 'SECURITY',    label: 'Sécurité',          emoji: '🛡️', desc: 'Stabilité, prévisibilité et évitement des risques' },
  { id: 'AUTONOMY',    label: 'Autonomie',          emoji: '🦅', desc: 'Indépendance et liberté d\'action' },
  { id: 'RECOGNITION', label: 'Reconnaissance',    emoji: '⭐', desc: 'Validation et estime des autres' },
  { id: 'LEARNING',    label: 'Apprentissage',     emoji: '📚', desc: 'Connaissances et développement personnel' },
  { id: 'HELPING',     label: 'Aider les autres',  emoji: '❤️', desc: 'Aider et soutenir les autres' },
];

const BIASES = [
  { id: 'CONFIRMATION',  label: 'Biais de confirmation', emoji: '🔄', desc: 'Cherche et interprète les infos qui confirment ses croyances' },
  { id: 'ANCHORING',     label: 'Ancrage cognitif',       emoji: '⚓', desc: 'Se focalise sur la première information reçue' },
  { id: 'AVAILABILITY',  label: 'Disponibilité',           emoji: '📱', desc: 'Surestime la probabilité d\'événements récents' },
  { id: 'SUNK_COST',     label: 'Coût irrécupérable',     emoji: '💸', desc: 'Poursuit un investissement à cause des ressources déjà engagées' },
  { id: 'DUNNING_KRUGER',label: 'Dunning-Kruger',         emoji: '🎭', desc: 'Les incompétents surestiment leurs compétences, les experts les sous-estiment' },
  { id: 'LOSS_AVERSION', label: 'Aversion aux pertes',    emoji: '😰', desc: 'Préfère éviter les pertes plutôt que chercher des gains' },
  { id: 'SOCIAL_PROOF',  label: 'Preuve sociale',         emoji: '👥', desc: 'Se conforme aux comportements du groupe' },
  { id: 'AUTHORITY',     label: 'Autorité',                emoji: '🎖️', desc: 'Confiance excessive aux figures d\'autorité' },
  { id: 'RECENCY',       label: 'Récence',                 emoji: '⏰', desc: 'Accorde plus d\'importance aux informations récentes' },
  { id: 'IN_GROUP',      label: 'Endogroupe',             emoji: '🏠', desc: 'Favorise les membres de son propre groupe' },
];

let OCEAN_DESCRIPTIONS = {
  O: { high: 'très ouvert aux nouvelles idées, créatif et curieux', low: 'pragmatique, préfère les routines et le concret' },
  C: { high: 'organisé, fiable, orienté résultats et détails', low: 'flexible et spontané, peut manquer de rigueur' },
  E: { high: 'extraverti, énergique, cherche la stimulation sociale', low: 'introverti, réfléchi, préfère les interactions limitées' },
  A: { high: 'coopératif, empathique, cherche l\'harmonie', low: 'direct voire abrasif, met ses objectifs avant les relations' },
  N: { high: 'émotionnellement réactif, stressable, sensible aux critiques', low: 'stable émotionnellement, calme sous pression' },
};

// ── DEMO DATA ─────────────────────────────────────────────
const DEMO_PERSON = {
  id: 'demo-001',
  name: 'Alexandre Dubois',
  role: 'Directeur Commercial',
  context: 'Contexte pro · Partenaire',
  avatarEmoji: '🧠',
  tags: ['Business', 'Décideur', 'Négociateur'],
  motivations: [
    { type: 'POWER',       intensity: 9, notes: 'Cherche toujours à être en position de force' },
    { type: 'RECOGNITION', intensity: 7, notes: 'A besoin de validation publique de ses succès' },
    { type: 'ACHIEVEMENT', intensity: 8, notes: 'Très orienté résultats et KPIs' },
  ],
  biases: [
    { type: 'ANCHORING',      intensity: 8, evidence: 'Reste bloqué sur le premier chiffre annoncé en négo' },
    { type: 'CONFIRMATION',   intensity: 6, evidence: 'Ignore les données qui contredisent sa vision' },
    { type: 'LOSS_AVERSION',  intensity: 7, evidence: 'Très réticent à annuler des engagements déjà pris' },
  ],
  ocean: { O: 8, C: 6, E: 9, A: 4, N: 5 },
  predictions: [
    {
      id: 'p1',
      context: 'Réunion budget Q3 — annonce de coupe budgétaire',
      predictedOutcome: 'Va tenter de négocier pour garder son budget, en mettant en avant les résultats passés.',
      actualOutcome: 'A négocié comme prévu, mais a aussi attaqué les autres départements.',
      accuracy: 7,
      createdAt: Date.now() - 864e5 * 10,
      resolved: true,
    },
    {
      id: 'p2',
      context: 'Présentation du nouveau concurrent X au marché',
      predictedOutcome: 'Va minimiser la menace par biais de confirmation. Dira que "notre produit est différent".',
      actualOutcome: null,
      accuracy: null,
      createdAt: Date.now() - 864e5 * 2,
      resolved: false,
    },
  ],
  accuracyScore: 82,
};

// ── Storage helpers ───────────────────────────────────────
const Storage = {
  key: 'pm_persons_v1',
  getAll() {
    try { return JSON.parse(localStorage.getItem(this.key) || '[]'); } catch { return []; }
  },
  save(persons) {
    localStorage.setItem(this.key, JSON.stringify(persons));
  },
  getCurrent() {
    const id = sessionStorage.getItem('pm_current_id') || 'demo-001';
    const all = this.getAll();
    return all.find(p => p.id === id) || DEMO_PERSON;
  },
  saveCurrent(person) {
    const all = this.getAll().filter(p => p.id !== person.id);
    all.unshift(person);
    this.save(all);
    sessionStorage.setItem('pm_current_id', person.id);
  },
};

// Seed demo data on first load
if (!localStorage.getItem('pm_persons_v1')) {
  Storage.save([DEMO_PERSON]);
}
