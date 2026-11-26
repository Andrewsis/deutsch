-- Lexicon (full-form dictionary + frequency lists) and example sentences for verbs.

-- Full-form dictionary (Morphy / LanguageTool german-pos-dict): every inflected form -> lemma + tags.
-- tags look like VER:SIN:SFT:3:PRÄ, SUB:AKK:SIN:NEU, ADJ:AKK:PLU:FEM:GRU:DEF; pos is the first segment.
-- prefix is the separable prefix of the lemma ('ab' for abwarten), NULL otherwise.
CREATE TABLE IF NOT EXISTS `word_forms` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `form` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `lemma` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `pos` varchar(8) NOT NULL,
  `tags` varchar(100) NOT NULL,
  `prefix` varchar(30) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_form` (`form`),
  KEY `idx_lemma` (`lemma`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- DeReWo lemma list (IDS Mannheim, 320k lemmas): frequency class 0 = most frequent.
CREATE TABLE IF NOT EXISTS `lemma_frequency` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `lemma` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `freq_class` tinyint(4) NOT NULL,
  `pos` varchar(10) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_lemma` (`lemma`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- DeReKo-2014-II word-form frequencies (top 100k forms, STTS tags); rank 1 = most frequent.
CREATE TABLE IF NOT EXISTS `form_frequency` (
  `rank` int(11) NOT NULL AUTO_INCREMENT,
  `form` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `lemma` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `stts` varchar(10) NOT NULL,
  `freq` double NOT NULL,
  PRIMARY KEY (`rank`),
  KEY `idx_form` (`form`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- German example sentences. source_ref: Tatoeba sentence id or Wiktionary page title;
-- citation: bibliographic reference of a Wiktionary quotation; sense: meaning gloss from Wiktionary.
-- rarity: worst (largest) form_frequency rank among the words of the sentence; 100001 = a word outside the top 100k.
CREATE TABLE IF NOT EXISTS `example_sentences` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `source` enum('tatoeba','wiktionary_en','wiktionary_de') NOT NULL,
  `source_ref` varchar(150) NOT NULL,
  `text` text NOT NULL,
  `author` varchar(100) DEFAULT NULL,
  `citation` text DEFAULT NULL,
  `sense` text DEFAULT NULL,
  `word_count` smallint(6) NOT NULL,
  `rarity` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_source_text` (`source`,`source_ref`,`text`(191))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- Translations of example sentences; a missing row means "not translated yet".
CREATE TABLE IF NOT EXISTS `example_translations` (
  `example_id` int(11) NOT NULL,
  `lang` char(2) NOT NULL,
  `text` text NOT NULL,
  `origin` enum('tatoeba','wiktionary','claude') NOT NULL,
  PRIMARY KEY (`example_id`,`lang`),
  CONSTRAINT `fk_et_example` FOREIGN KEY (`example_id`) REFERENCES `example_sentences` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- Which verb an example illustrates; form = the matched words to highlight (e.g. 'wartet … ab').
CREATE TABLE IF NOT EXISTS `verb_examples` (
  `verb_id` int(11) NOT NULL,
  `example_id` int(11) NOT NULL,
  `form` varchar(200) NOT NULL,
  PRIMARY KEY (`verb_id`,`example_id`),
  KEY `idx_example` (`example_id`),
  CONSTRAINT `fk_ve_verb` FOREIGN KEY (`verb_id`) REFERENCES `verben_wide` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_ve_example` FOREIGN KEY (`example_id`) REFERENCES `example_sentences` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
