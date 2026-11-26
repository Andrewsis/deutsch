/*M!999999\- enable the sandbox mode */ 

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;
DROP TABLE IF EXISTS `verben_wide`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `verben_wide` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `lemma` varchar(100) NOT NULL,
  `aux` varchar(100) DEFAULT NULL,
  `prs1sg` varchar(100) DEFAULT NULL,
  `prs2sg` varchar(100) DEFAULT NULL,
  `prs3sg` varchar(100) DEFAULT NULL,
  `prs1pl` varchar(100) DEFAULT NULL,
  `prs2pl` varchar(100) DEFAULT NULL,
  `prs3pl` varchar(100) DEFAULT NULL,
  `prt1sg` varchar(100) DEFAULT NULL,
  `prt2sg` varchar(100) DEFAULT NULL,
  `prt3sg` varchar(100) DEFAULT NULL,
  `prt1pl` varchar(100) DEFAULT NULL,
  `prt2pl` varchar(100) DEFAULT NULL,
  `prt3pl` varchar(100) DEFAULT NULL,
  `kj1_1sg` varchar(100) DEFAULT NULL,
  `kj1_2sg` varchar(100) DEFAULT NULL,
  `kj1_3sg` varchar(100) DEFAULT NULL,
  `kj1_1pl` varchar(100) DEFAULT NULL,
  `kj1_2pl` varchar(100) DEFAULT NULL,
  `kj1_3pl` varchar(100) DEFAULT NULL,
  `kj2_1sg` varchar(100) DEFAULT NULL,
  `kj2_2sg` varchar(100) DEFAULT NULL,
  `kj2_3sg` varchar(100) DEFAULT NULL,
  `kj2_1pl` varchar(100) DEFAULT NULL,
  `kj2_2pl` varchar(100) DEFAULT NULL,
  `kj2_3pl` varchar(100) DEFAULT NULL,
  `fut1_1sg` varchar(100) DEFAULT NULL,
  `fut1_2sg` varchar(100) DEFAULT NULL,
  `fut1_3sg` varchar(100) DEFAULT NULL,
  `fut1_1pl` varchar(100) DEFAULT NULL,
  `fut1_2pl` varchar(100) DEFAULT NULL,
  `fut1_3pl` varchar(100) DEFAULT NULL,
  `fut2_1sg` varchar(100) DEFAULT NULL,
  `fut2_2sg` varchar(100) DEFAULT NULL,
  `fut2_3sg` varchar(100) DEFAULT NULL,
  `fut2_1pl` varchar(100) DEFAULT NULL,
  `fut2_2pl` varchar(100) DEFAULT NULL,
  `fut2_3pl` varchar(100) DEFAULT NULL,
  `imp2sg` varchar(100) DEFAULT NULL,
  `imp2pl` varchar(100) DEFAULT NULL,
  `inf` varchar(100) DEFAULT NULL,
  `pa1` varchar(100) DEFAULT NULL,
  `pa2` varchar(100) DEFAULT NULL,
  `translation_ua` text DEFAULT NULL,
  `praepositionen_ua` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`praepositionen_ua`)),
  `audio_forms` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT '[]' CHECK (json_valid(`audio_forms`)),
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_lemma` (`lemma`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `verb_translations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `verb_translations` (
  `verb_id` int(11) NOT NULL,
  `lang` char(2) NOT NULL,
  `translation` text DEFAULT NULL,
  `prepositions` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`prepositions`)),
  PRIMARY KEY (`verb_id`,`lang`),
  CONSTRAINT `fk_vt_verb` FOREIGN KEY (`verb_id`) REFERENCES `verben_wide` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(50) NOT NULL,
  `email` varchar(255) NOT NULL,
  `password_hash` varchar(255) NOT NULL,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `user_flashcards`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `user_flashcards` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `verb_id` int(11) NOT NULL,
  `repetition` int(11) DEFAULT 0,
  `interval_days` int(11) DEFAULT 0,
  `ease_factor` float DEFAULT 2.5,
  `next_review` datetime DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_user_verb` (`user_id`,`verb_id`),
  KEY `fk_verb` (`verb_id`),
  CONSTRAINT `fk_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_verb` FOREIGN KEY (`verb_id`) REFERENCES `verben_wide` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `videos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `videos` (
  `id` varchar(11) NOT NULL,
  `views` bigint(20) unsigned NOT NULL DEFAULT 0,
  `upload_year` smallint(5) unsigned DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `subtitles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `subtitles` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `video_id` varchar(11) NOT NULL,
  `start_sec` decimal(8,2) NOT NULL,
  `text` text NOT NULL,
  PRIMARY KEY (`id`),
  KEY `video_id` (`video_id`),
  FULLTEXT KEY `idx_exact_search` (`text`),
  CONSTRAINT `1` FOREIGN KEY (`video_id`) REFERENCES `videos` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
DROP TABLE IF EXISTS `word_stats`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `word_stats` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `word` varchar(255) NOT NULL,
  `is_bigram` tinyint(1) NOT NULL DEFAULT 0,
  `count` int(11) NOT NULL DEFAULT 0,
  `scraped_at` datetime NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  KEY `idx_word` (`word`),
  KEY `idx_scraped_at` (`scraped_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
-- Lexicon (full-form dictionary + frequency lists) and example sentences for verbs.

-- Full-form dictionary (Morphy / LanguageTool german-pos-dict): every inflected form -> lemma + tags.
-- tags look like VER:SIN:SFT:3:PRÄ, SUB:AKK:SIN:NEU, ADJ:AKK:PLU:FEM:GRU:DEF; pos is the first segment.
-- prefix is the separable prefix of the lemma ('ab' for abwarten), NULL otherwise.
CREATE TABLE `word_forms` (
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
CREATE TABLE `lemma_frequency` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `lemma` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `freq_class` tinyint(4) NOT NULL,
  `pos` varchar(10) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_lemma` (`lemma`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- DeReKo-2014-II word-form frequencies (top 100k forms, STTS tags); rank 1 = most frequent.
CREATE TABLE `form_frequency` (
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
CREATE TABLE `example_sentences` (
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
CREATE TABLE `example_translations` (
  `example_id` int(11) NOT NULL,
  `lang` char(2) NOT NULL,
  `text` text NOT NULL,
  `origin` enum('tatoeba','wiktionary','claude') NOT NULL,
  PRIMARY KEY (`example_id`,`lang`),
  CONSTRAINT `fk_et_example` FOREIGN KEY (`example_id`) REFERENCES `example_sentences` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- Which verb an example illustrates; form = the matched words to highlight (e.g. 'wartet … ab').
CREATE TABLE `verb_examples` (
  `verb_id` int(11) NOT NULL,
  `example_id` int(11) NOT NULL,
  `form` varchar(200) NOT NULL,
  PRIMARY KEY (`verb_id`,`example_id`),
  KEY `idx_example` (`example_id`),
  CONSTRAINT `fk_ve_verb` FOREIGN KEY (`verb_id`) REFERENCES `verben_wide` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_ve_example` FOREIGN KEY (`example_id`) REFERENCES `example_sentences` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

