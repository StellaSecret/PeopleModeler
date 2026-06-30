// ── PEOPLE MODELER — i18n Layer ───────────────────────────

const LANG_KEY = 'pm_lang';

const FR = {
  lang: 'fr',
  // Navigation
  nav_title: '🧩 <span>People</span>Modeler',
  nav_home: 'Accueil',
  nav_demo: 'Démo',
  nav_app: 'App',
  nav_compare: 'Comparer',
  nav_open_app: 'Ouvrir l\'app',
  nav_back: '← Retour',
  nav_features: 'Fonctionnalités',
  nav_how: 'Comment',
  nav_web_badge: 'web',

  // Landing page
  hero_tag: '⚠️ Éthique d\'abord · Ultra-puissant ensuite',
  hero_title_1: 'Modéliser les gens',
  hero_title_2: 'comme des <em>systèmes</em>',
  hero_sub: 'Motivations · Biais · Comportements<br>Prédire avec précision. Comprendre avec empathie.',
  hero_cta_app: '🚀 Lancer l\'app web',
  hero_cta_demo: 'Voir la démo →',
  preview_name: 'Système : Alexandre D.',
  preview_role: 'Décideur · Contexte pro',
  preview_power: 'Pouvoir 👑',
  preview_anchor: 'Biais ancrage ⚓',
  preview_accuracy: 'Précision prédictions',
  preview_badge: '🔮 3 prédictions en attente',

  features_title: 'Ce que vous <em>gagnez</em>',
  f1_title: 'Fiches personnalisées',
  f1_desc: 'Chaque personne est un modèle : motivations profondes, biais cognitifs, patterns comportementaux. Un profil OCEAN complet.',
  f2_title: 'Prédictions comportementales',
  f2_desc: 'Anticipez les réactions dans chaque contexte — stress, conflit, succès, incertitude. Avec un indice de confiance.',
  f3_title: 'Feedback Loop',
  f3_desc: 'Notez la précision de vos prédictions. Le système apprend de vos retours. Votre modèle mental s\'affine.',
  f4_title: 'Insights & comparaisons',
  f4_desc: 'Comparez deux profils, identifiez les synergies et les points de friction. Parfait pour les équipes et les négociations.',
  f5_title: '100% local',
  f5_desc: 'Vos données restent sur votre appareil. Aucun serveur, aucune synchronisation cloud. Votre modèle mental est privé.',
  f6_title: 'Éthique by design',
  f6_desc: 'Rappels intégrés. Conçu pour améliorer vos relations, pas pour manipuler. Le pouvoir implique la responsabilité.',

  how_title: 'Comment ça <em>fonctionne</em>',
  step1_title: 'Créez une fiche',
  step1_desc: 'Nom, rôle, contexte. Choisissez un avatar. C\'est votre point de départ.',
  step2_title: 'Modélisez le système',
  step2_desc: 'Ajoutez les motivations (intensité 1–10), les biais observés, les patterns comportementaux selon le contexte.',
  step3_title: 'Prédisez & testez',
  step3_desc: 'Avant une réunion, un conflit, une négociation — notez votre prédiction. Observez. Notez la précision.',
  step4_title: 'Affinez le modèle',
  step4_desc: 'Le feedback loop améliore votre compréhension. Avec le temps, vous prédisez à 80%+.',

  usecases_title: 'Ultra-puissant pour…',
  uc1_title: 'Business',
  uc1_desc: 'Négociations, clients, partenaires. Anticipez les objections, adaptez votre approche.',
  uc2_title: 'Relations',
  uc2_desc: 'Comprenez vos proches en profondeur. Réduisez les conflits. Créez des connexions durables.',
  uc3_title: 'Stratégie',
  uc3_desc: 'Leadership, politique d\'équipe, gestion de conflits. Prenez des décisions informées.',
  uc4_title: 'Introspection',
  uc4_desc: 'Modélisez-vous vous-même. Identifiez vos propres biais. Devenez plus conscient.',

  ethics_title: 'Note éthique importante',
  ethics_desc: 'People Modeler est un outil de compréhension, pas de manipulation. Utilisez-le pour <strong>améliorer vos relations</strong>, pas pour exploiter les failles des autres. La connaissance des systèmes humains est une responsabilité.',

  cta_title: 'Prêt à comprendre les systèmes humains ?',
  cta_sub: 'Application web. Données 100% locales. Gratuit.',
  cta_app_btn: '🚀 Lancer l\'application',
  cta_demo_btn: '👁️ Voir la démo',

  footer_copy: 'Open Source · MIT License · À utiliser éthiquement',

  // App page
  sidebar_title: 'Profils',
  sidebar_new: 'Nouveau profil',
  sidebar_empty: 'Aucun profil.<br>Cliquez sur + pour commencer.',
  sidebar_gdrive_off: 'Sync Google Drive',
  sidebar_gdrive_on: '✓ Drive connecté',
  sidebar_export: 'Exporter JSON',
  sidebar_import: 'Importer JSON',

  empty_title: 'Aucun profil',
  empty_desc: 'Créez votre premier profil pour commencer à modéliser les systèmes humains autour de vous.',
  empty_cta: '+ Créer un profil',
  empty_or: 'ou',
  empty_gdrive: '☁️ Connecter Google Drive',
  empty_hint: '💾 Vos données sont stockées localement dans votre navigateur.<br>Connectez Google Drive pour les synchroniser avec l\'app mobile.',

  precision_label: 'précision',
  edit_btn: '✏️ Éditer',
  delete_profile_title: 'Supprimer le profil',

  tab_motivations: '💡 Motivations',
  tab_biases: '🧠 Biais',
  tab_ocean: '🌊 OCEAN',
  tab_predictions: '🔮 Prédictions',
  tab_insights: '✨ Insights',

  mot_section_title: 'Motivations profondes',
  mot_empty: 'Aucune motivation ajoutée.',
  mot_add: '+ Ajouter une motivation',
  mot_dialog_title: '💡 Ajouter une motivation',
  mot_type_label: 'Type',
  mot_intensity_label: 'Intensité',
  mot_notes_label: 'Notes (optionnel)',
  mot_notes_placeholder: 'Comportement observé…',
  mot_delete_label: 'Supprimer',

  bias_section_title: 'Biais cognitifs observés',
  bias_empty: 'Aucun biais ajouté.',
  bias_add: '+ Ajouter un biais',
  bias_dialog_title: '🧠 Ajouter un biais',
  bias_type_label: 'Type',
  bias_intensity_label: 'Intensité observée',
  bias_evidence_label: 'Preuve / exemple',
  bias_evidence_placeholder: 'Situation concrète observée…',

  ocean_section_title: 'Profil de personnalité (Big Five)',
  ocean_interp_default: 'Ajustez les curseurs pour voir l\'interprétation.',

  pred_section_title: 'Prédictions comportementales',
  pred_empty: 'Aucune prédiction.',
  pred_form_title: 'Nouvelle prédiction',
  pred_ctx_label: 'Contexte / Situation',
  pred_ctx_placeholder: 'Ex: Réunion de budget vendredi…',
  pred_out_label: 'Comportement prédit',
  pred_out_placeholder: 'Je prédis qu\'il va…',
  pred_save: '🔮 Enregistrer',
  pred_pending: '⏳ En attente',
  pred_resolved: '✅ Résolue',
  pred_accuracy: 'Précision',
  pred_resolve_btn: 'Résoudre →',
  pred_resolve_title: '✅ Résoudre la prédiction',
  pred_resolve_actual_label: 'Ce qui s\'est réellement passé',
  pred_resolve_actual_placeholder: 'Résultat réel…',
  pred_resolve_acc_label: 'Précision',

  insight_section_title: 'Analyse comportementale par contexte',
  insight_placeholder: '← Sélectionnez un contexte',
  insight_context_label: 'Analyse comportementale — contexte',
  insight_stress: '😰 Sous stress',
  insight_conflict: '⚔️ En conflit',
  insight_success: '🏆 En réussite',
  insight_uncertainty: '❓ Incertitude',
  insight_recognition: '⭐ Reconnaissance',
  insight_threat: '🛡️ Menacé·e',

  modal_cancel: 'Annuler',
  modal_confirm: 'Confirmer',

  // Profile form
  form_name_label: 'Prénom / Nom',
  form_name_placeholder: 'Ex: Marie Dupont',
  form_role_label: 'Rôle / Poste',
  form_role_placeholder: 'Ex: DRH, Coach, Ami·e',
  form_context_label: 'Contexte',
  form_context_placeholder: 'Ex: Professionnel, Personnel',
  form_avatar_label: 'Avatar',
  form_new_title: '✨ Nouveau profil',
  form_edit_title: '✏️ Modifier le profil',
  form_alert_name: 'Donnez un nom au profil.',

  // Delete
  delete_confirm_title: 'Supprimer',
  delete_confirm_msg: 'Irréversible.',
  edit_delete_confirm: 'Cette action est irréversible.',

  // Predictions alert
  pred_alert_fill: 'Remplissez le contexte et la prédiction.',
  pred_alert_describe: 'Décrivez ce qui s\'est passé.',
  pred_insight_context: 'Contexte',

  // Sync
  gdrive_connect_title: '☁️ Connecter Google Drive',
  gdrive_connect_desc: 'Synchronisez vos profils avec l\'app mobile via votre Google Drive personnel.<br>Les données sont stockées dans <code style="color:var(--cyan)">PeopleModeler/data.json</code>.',
  gdrive_oauth_wip: '⚠️ L\'intégration OAuth Google sera connectée lors de la release.<br>En attendant, utilisez <strong>Export / Import JSON</strong>.',
  gdrive_simulate: '🔗 Simuler la connexion',
  gdrive_disconnect_confirm: 'Déconnecter Google Drive ?',
  gdrive_banner: '☁️ Sync Google Drive active — <code>PeopleModeler/data.json</code>',

  // Compare page
  compare_title: 'Comparaison de profils',
  compare_sub: 'Identifiez synergies et points de friction entre deux personnes',
  compare_vs: 'VS',
  compare_synergy: 'synergie',
  compare_analysis_title: '🔍 Analyse de la dynamique',
  compare_synergies: '✅ Synergies',
  compare_friction: '⚠️ Points de friction',
  compare_strategy: '♟️ Stratégie d\'interaction',
  compare_ethics: '⚠️ Ces analyses sont des <em>modèles probabilistes</em>, pas des vérités absolues. Utilisez-les pour mieux comprendre, jamais pour manipuler.',
  compare_top_mot: '💡 Top Motivation',
  compare_bias_main: '🧠 Biais principal',
  compare_ocean: '🌊 Profil OCEAN',

  // Person page (demo)
  person_title: 'Fiche Personne — People Modeler',
  person_accuracy: 'précision',
  person_pred_add: '🔮 Enregistrer la prédiction',
  person_pred_placeholder: 'Sélectionnez un contexte pour voir l\'analyse',
  person_modal_title: 'Ajouter',
  person_insight_label: 'Analyse comportementale',

  // OCEAN labels
  ocean_o: 'O — Ouverture',
  ocean_c: 'C — Consciencieux',
  ocean_e: 'E — Extraversion',
  ocean_a: 'A — Agréabilité',
  ocean_n: 'N — Névrosisme',

  // Motivations enum labels
  mot_power: 'Pouvoir',
  mot_achievement: 'Accomplissement',
  mot_affiliation: 'Appartenance',
  mot_security: 'Sécurité',
  mot_autonomy: 'Autonomie',
  mot_recognition: 'Reconnaissance',
  mot_learning: 'Apprentissage',
  mot_helping: 'Aider les autres',

  // Biases enum labels
  bias_confirmation: 'Biais de confirmation',
  bias_anchoring: 'Ancrage cognitif',
  bias_availability: 'Disponibilité',
  bias_sunk_cost: 'Coût irrécupérable',
  bias_dunning_kruger: 'Dunning-Kruger',
  bias_loss_aversion: 'Aversion aux pertes',
  bias_social_proof: 'Preuve sociale',
  bias_authority: 'Autorité',
  bias_recency: 'Récence',
  bias_in_group: 'Endogroupe',

  // OCEAN descriptions
  ocean_o_high: 'très ouvert aux nouvelles idées, créatif et curieux',
  ocean_o_low: 'pragmatique, préfère les routines et le concret',
  ocean_c_high: 'organisé, fiable, orienté résultats et détails',
  ocean_c_low: 'flexible et spontané, peut manquer de rigueur',
  ocean_e_high: 'extraverti, énergique, cherche la stimulation sociale',
  ocean_e_low: 'introverti, réfléchi, préfère les interactions limitées',
  ocean_a_high: 'coopératif, empathique, cherche l\'harmonie',
  ocean_a_low: 'direct voire abrasif, met ses objectifs avant les relations',
  ocean_n_high: 'émotionnellement réactif, stressable, sensible aux critiques',
  ocean_n_low: 'stable émotionnellement, calme sous pression',

  // App insights
  insight_stress_header: 'sous stress',
  insight_conflict_header: 'en conflit',
  insight_success_header: 'en réussite',
  insight_uncertainty_header: 'dans l\'incertitude',
  insight_recognition_header: 'cherchant la reconnaissance',
  insight_threat_header: 'se sentant menacé·e',

  // Format strings
  score_format: '{score}%',
  intensity_format: '{value}/10',
  pending_count_format: '⏳ {count} prédiction(s) en attente de résolution',
};

const EN = {
  lang: 'en',
  nav_title: '🧩 <span>People</span>Modeler',
  nav_home: 'Home',
  nav_demo: 'Demo',
  nav_app: 'App',
  nav_compare: 'Compare',
  nav_open_app: 'Open App',
  nav_back: '← Back',
  nav_features: 'Features',
  nav_how: 'How',
  nav_web_badge: 'web',

  hero_tag: '⚠️ Ethics first · Ultra-powerful second',
  hero_title_1: 'Model people',
  hero_title_2: 'like <em>systems</em>',
  hero_sub: 'Motivations · Biases · Behaviors<br>Predict with accuracy. Understand with empathy.',
  hero_cta_app: '🚀 Launch Web App',
  hero_cta_demo: 'View Demo →',
  preview_name: 'System: Alexandre D.',
  preview_role: 'Decision-maker · Pro context',
  preview_power: 'Power 👑',
  preview_anchor: 'Anchoring bias ⚓',
  preview_accuracy: 'Prediction accuracy',
  preview_badge: '🔮 3 pending predictions',

  features_title: 'What you <em>gain</em>',
  f1_title: 'Custom Profiles',
  f1_desc: 'Each person is a model: deep motivations, cognitive biases, behavioral patterns. A complete OCEAN profile.',
  f2_title: 'Behavioral Predictions',
  f2_desc: 'Anticipate reactions in every context — stress, conflict, success, uncertainty. With a confidence score.',
  f3_title: 'Feedback Loop',
  f3_desc: 'Rate your prediction accuracy. The system learns from your feedback. Your mental model sharpens over time.',
  f4_title: 'Insights & Comparisons',
  f4_desc: 'Compare two profiles, identify synergies and friction points. Perfect for teams and negotiations.',
  f5_title: '100% Local',
  f5_desc: 'Your data stays on your device. No server, no cloud sync. Your mental model stays private.',
  f6_title: 'Ethics by Design',
  f6_desc: 'Built-in reminders. Designed to improve your relationships, not to manipulate. Power comes with responsibility.',

  how_title: 'How it <em>works</em>',
  step1_title: 'Create a profile',
  step1_desc: 'Name, role, context. Pick an avatar. That\'s your starting point.',
  step2_title: 'Model the system',
  step2_desc: 'Add motivations (intensity 1–10), observed biases, behavioral patterns by context.',
  step3_title: 'Predict & test',
  step3_desc: 'Before a meeting, conflict, negotiation — write your prediction. Observe. Rate accuracy.',
  step4_title: 'Refine the model',
  step4_desc: 'The feedback loop improves your understanding. Over time, predict at 80%+ accuracy.',

  usecases_title: 'Ultra-powerful for…',
  uc1_title: 'Business',
  uc1_desc: 'Negotiations, clients, partners. Anticipate objections, adapt your approach.',
  uc2_title: 'Relationships',
  uc2_desc: 'Understand your loved ones deeply. Reduce conflicts. Build lasting connections.',
  uc3_title: 'Strategy',
  uc3_desc: 'Leadership, team politics, conflict management. Make informed decisions.',
  uc4_title: 'Introspection',
  uc4_desc: 'Model yourself. Identify your own biases. Become more self-aware.',

  ethics_title: 'Important Ethical Note',
  ethics_desc: 'People Modeler is a tool for understanding, not manipulation. Use it to <strong>improve your relationships</strong>, not to exploit others\' weaknesses. Knowledge of human systems is a responsibility.',

  cta_title: 'Ready to understand human systems?',
  cta_sub: 'Web app. 100% local data. Free.',
  cta_app_btn: '🚀 Launch the App',
  cta_demo_btn: '👁️ View Demo',

  footer_copy: 'Open Source · MIT License · Use ethically',

  sidebar_title: 'Profiles',
  sidebar_new: 'New profile',
  sidebar_empty: 'No profiles.<br>Click + to start.',
  sidebar_gdrive_off: 'Sync Google Drive',
  sidebar_gdrive_on: '✓ Drive connected',
  sidebar_export: 'Export JSON',
  sidebar_import: 'Import JSON',

  empty_title: 'No profile',
  empty_desc: 'Create your first profile to start modeling the human systems around you.',
  empty_cta: '+ Create profile',
  empty_or: 'or',
  empty_gdrive: '☁️ Connect Google Drive',
  empty_hint: '💾 Your data is stored locally in your browser.<br>Connect Google Drive to sync with the mobile app.',

  precision_label: 'accuracy',
  edit_btn: '✏️ Edit',
  delete_profile_title: 'Delete profile',

  tab_motivations: '💡 Motivations',
  tab_biases: '🧠 Biases',
  tab_ocean: '🌊 OCEAN',
  tab_predictions: '🔮 Predictions',
  tab_insights: '✨ Insights',

  mot_section_title: 'Deep Motivations',
  mot_empty: 'No motivations added.',
  mot_add: '+ Add motivation',
  mot_dialog_title: '💡 Add a motivation',
  mot_type_label: 'Type',
  mot_intensity_label: 'Intensity',
  mot_notes_label: 'Notes (optional)',
  mot_notes_placeholder: 'Observed behavior…',
  mot_delete_label: 'Delete',

  bias_section_title: 'Observed Cognitive Biases',
  bias_empty: 'No biases added.',
  bias_add: '+ Add a bias',
  bias_dialog_title: '🧠 Add a bias',
  bias_type_label: 'Type',
  bias_intensity_label: 'Observed intensity',
  bias_evidence_label: 'Evidence / example',
  bias_evidence_placeholder: 'Observed concrete situation…',

  ocean_section_title: 'Personality Profile (Big Five)',
  ocean_interp_default: 'Adjust the sliders to see the interpretation.',

  pred_section_title: 'Behavioral Predictions',
  pred_empty: 'No predictions.',
  pred_form_title: 'New prediction',
  pred_ctx_label: 'Context / Situation',
  pred_ctx_placeholder: 'Ex: Friday budget meeting…',
  pred_out_label: 'Predicted behavior',
  pred_out_placeholder: 'I predict they will…',
  pred_save: '🔮 Save',
  pred_pending: '⏳ Pending',
  pred_resolved: '✅ Resolved',
  pred_accuracy: 'Accuracy',
  pred_resolve_btn: 'Resolve →',
  pred_resolve_title: '✅ Resolve prediction',
  pred_resolve_actual_label: 'What actually happened',
  pred_resolve_actual_placeholder: 'Actual result…',
  pred_resolve_acc_label: 'Accuracy',

  insight_section_title: 'Behavioral Analysis by Context',
  insight_placeholder: '← Select a context',
  insight_context_label: 'Behavioral analysis — context',
  insight_stress: '😰 Under stress',
  insight_conflict: '⚔️ In conflict',
  insight_success: '🏆 In success',
  insight_uncertainty: '❓ Uncertainty',
  insight_recognition: '⭐ Seeking recognition',
  insight_threat: '🛡️ Feeling threatened',

  modal_cancel: 'Cancel',
  modal_confirm: 'Confirm',

  form_name_label: 'First / Last Name',
  form_name_placeholder: 'Ex: Marie Curie',
  form_role_label: 'Role / Position',
  form_role_placeholder: 'Ex: VP, Coach, Friend',
  form_context_label: 'Context',
  form_context_placeholder: 'Ex: Professional, Personal',
  form_avatar_label: 'Avatar',
  form_new_title: '✨ New profile',
  form_edit_title: '✏️ Edit profile',
  form_alert_name: 'Please enter a name for the profile.',

  delete_confirm_title: 'Delete',
  delete_confirm_msg: 'Irreversible.',
  edit_delete_confirm: 'This action is irreversible.',

  pred_alert_fill: 'Please fill in the context and prediction.',
  pred_alert_describe: 'Please describe what happened.',
  pred_insight_context: 'Context',

  gdrive_connect_title: '☁️ Connect Google Drive',
  gdrive_connect_desc: 'Sync your profiles with the mobile app via your personal Google Drive.<br>Data is stored in <code style="color:var(--cyan)">PeopleModeler/data.json</code>.',
  gdrive_oauth_wip: '⚠️ Google OAuth integration will be connected at release.<br>Until then, use <strong>Export / Import JSON</strong>.',
  gdrive_simulate: '🔗 Simulate connection',
  gdrive_disconnect_confirm: 'Disconnect Google Drive?',
  gdrive_banner: '☁️ Google Drive sync active — <code>PeopleModeler/data.json</code>',

  compare_title: 'Profile Comparison',
  compare_sub: 'Identify synergies and friction points between two people',
  compare_vs: 'VS',
  compare_synergy: 'synergy',
  compare_analysis_title: '🔍 Dynamic Analysis',
  compare_synergies: '✅ Synergies',
  compare_friction: '⚠️ Friction Points',
  compare_strategy: '♟️ Interaction Strategy',
  compare_ethics: '⚠️ These analyses are <em>probabilistic models</em>, not absolute truths. Use them to better understand, never to manipulate.',
  compare_top_mot: '💡 Top Motivation',
  compare_bias_main: '🧠 Main Bias',
  compare_ocean: '🌊 OCEAN Profile',

  person_title: 'Person Profile — People Modeler',
  person_accuracy: 'accuracy',
  person_pred_add: '🔮 Save prediction',
  person_pred_placeholder: 'Select a context to view analysis',
  person_modal_title: 'Add',
  person_insight_label: 'Behavioral analysis',

  ocean_o: 'O — Openness',
  ocean_c: 'C — Conscientiousness',
  ocean_e: 'E — Extraversion',
  ocean_a: 'A — Agreeableness',
  ocean_n: 'N — Neuroticism',

  mot_power: 'Power',
  mot_achievement: 'Achievement',
  mot_affiliation: 'Affiliation',
  mot_security: 'Security',
  mot_autonomy: 'Autonomy',
  mot_recognition: 'Recognition',
  mot_learning: 'Learning',
  mot_helping: 'Helping others',

  bias_confirmation: 'Confirmation bias',
  bias_anchoring: 'Cognitive anchoring',
  bias_availability: 'Availability',
  bias_sunk_cost: 'Sunk cost',
  bias_dunning_kruger: 'Dunning-Kruger',
  bias_loss_aversion: 'Loss aversion',
  bias_social_proof: 'Social proof',
  bias_authority: 'Authority',
  bias_recency: 'Recency',
  bias_in_group: 'In-group',

  ocean_o_high: 'very open to new ideas, creative and curious',
  ocean_o_low: 'pragmatic, prefers routines and concrete things',
  ocean_c_high: 'organized, reliable, results and detail-oriented',
  ocean_c_low: 'flexible and spontaneous, may lack rigor',
  ocean_e_high: 'extraverted, energetic, seeks social stimulation',
  ocean_e_low: 'introverted, thoughtful, prefers limited interactions',
  ocean_a_high: 'cooperative, empathetic, seeks harmony',
  ocean_a_low: 'direct or abrasive, puts goals before relationships',
  ocean_n_high: 'emotionally reactive, prone to stress, sensitive to criticism',
  ocean_n_low: 'emotionally stable, calm under pressure',

  insight_stress_header: 'under stress',
  insight_conflict_header: 'in conflict',
  insight_success_header: 'in success',
  insight_uncertainty_header: 'in uncertainty',
  insight_recognition_header: 'seeking recognition',
  insight_threat_header: 'feeling threatened',

  score_format: '{score}%',
  intensity_format: '{value}/10',
  pending_count_format: '⏳ {count} prediction(s) pending',
};

// Current language
let _currentLang = null;

function getLang() {
  if (_currentLang) return _currentLang;
  const stored = localStorage.getItem(LANG_KEY);
  _currentLang = stored === 'en' ? EN : FR;
  return _currentLang;
}

function setLang(lang) {
  _currentLang = lang === 'en' ? EN : FR;
  localStorage.setItem(LANG_KEY, lang);
  document.documentElement.lang = lang === 'en' ? 'en' : 'fr';
  // Re-translate page
  translatePage();
  // Re-render app if on app page
  if (typeof renderSidebar === 'function') {
    renderSidebar();
    renderMain();
  }
  if (typeof renderAll === 'function') {
    renderAll();
    if (typeof updateOceanInterpretation === 'function') updateOceanInterpretation();
  }
  const L = getLang();
  // Update OCEAN descriptions reference
  if (typeof OCEAN_DESCRIPTIONS !== 'undefined') {
    OCEAN_DESCRIPTIONS = {
      O: { high: L.ocean_o_high, low: L.ocean_o_low },
      C: { high: L.ocean_c_high, low: L.ocean_c_low },
      E: { high: L.ocean_e_high, low: L.ocean_e_low },
      A: { high: L.ocean_a_high, low: L.ocean_a_low },
      N: { high: L.ocean_n_high, low: L.ocean_n_low },
    };
  }
  // Update enum labels
  if (typeof MOTIVATIONS !== 'undefined') {
    MOTIVATIONS.forEach(m => {
      const key = 'mot_' + m.id.toLowerCase();
      if (L[key]) m.label = L[key];
    });
  }
  if (typeof BIASES !== 'undefined') {
    BIASES.forEach(b => {
      const key = 'bias_' + b.id.toLowerCase();
      if (L[key]) b.label = L[key];
    });
  }
  // Re-translate compare page
  if (typeof renderCompare === 'function') {
    renderCompare();
  }
}

function t(key) {
  const L = getLang();
  return L[key] || key;
}

function translatePage() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    const raw = t(key);
    if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
      el.setAttribute('placeholder', raw);
    } else {
      el.innerHTML = raw;
    }
  });
}

function toggleLang() {
  const current = getLang().lang;
  const next = current === 'fr' ? 'en' : 'fr';
  setLang(next);
}

function initI18n() {
  const stored = localStorage.getItem(LANG_KEY) || 'fr';
  setLang(stored);
}

// Auto-init
document.addEventListener('DOMContentLoaded', initI18n);
