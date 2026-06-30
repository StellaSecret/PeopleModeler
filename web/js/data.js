// ── PEOPLE MODELER — Data & Constants ────────────────────

const MOTIVATIONS = [
  { id: 'POWER',       label: 'Pouvoir',          emoji: '👑' },
  { id: 'ACHIEVEMENT', label: 'Accomplissement',   emoji: '🏆' },
  { id: 'AFFILIATION', label: 'Appartenance',      emoji: '🤝' },
  { id: 'SECURITY',    label: 'Sécurité',          emoji: '🛡️' },
  { id: 'AUTONOMY',    label: 'Autonomie',          emoji: '🦅' },
  { id: 'RECOGNITION', label: 'Reconnaissance',    emoji: '⭐' },
  { id: 'LEARNING',    label: 'Apprentissage',     emoji: '📚' },
  { id: 'HELPING',     label: 'Aider les autres',  emoji: '❤️' },
];

const BIASES = [
  { id: 'CONFIRMATION',  label: 'Biais de confirmation', emoji: '🔄' },
  { id: 'ANCHORING',     label: 'Ancrage cognitif',       emoji: '⚓' },
  { id: 'AVAILABILITY',  label: 'Disponibilité',           emoji: '📱' },
  { id: 'SUNK_COST',     label: 'Coût irrécupérable',     emoji: '💸' },
  { id: 'DUNNING_KRUGER',label: 'Dunning-Kruger',         emoji: '🎭' },
  { id: 'LOSS_AVERSION', label: 'Aversion aux pertes',    emoji: '😰' },
  { id: 'SOCIAL_PROOF',  label: 'Preuve sociale',         emoji: '👥' },
  { id: 'AUTHORITY',     label: 'Autorité',                emoji: '🎖️' },
  { id: 'RECENCY',       label: 'Récence',                 emoji: '⏰' },
  { id: 'IN_GROUP',      label: 'Endogroupe',             emoji: '🏠' },
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
