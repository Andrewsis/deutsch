-- Per-language verb translations (uk, en, ...). Replaces verben_wide.translation_ua / praepositionen_ua.
CREATE TABLE IF NOT EXISTS `verb_translations` (
  `verb_id` int(11) NOT NULL,
  `lang` char(2) NOT NULL,
  `translation` text DEFAULT NULL,
  `prepositions` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL CHECK (json_valid(`prepositions`)),
  PRIMARY KEY (`verb_id`,`lang`),
  CONSTRAINT `fk_vt_verb` FOREIGN KEY (`verb_id`) REFERENCES `verben_wide` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- Copy existing Ukrainian data; preposition objects {phrase, ua} become {phrase, text}.
INSERT INTO `verb_translations` (`verb_id`, `lang`, `translation`, `prepositions`)
SELECT `id`, 'uk', `translation_ua`, REPLACE(`praepositionen_ua`, '"ua":', '"text":')
FROM `verben_wide`
ON DUPLICATE KEY UPDATE
  `translation` = VALUES(`translation`),
  `prepositions` = VALUES(`prepositions`);
