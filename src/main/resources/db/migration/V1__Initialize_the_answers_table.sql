CREATE TABLE answers (
                                       id                       INTEGER PRIMARY KEY AUTOINCREMENT,
                                       entry_date               TEXT DEFAULT (date('now')) NOT NULL,
                                       bedtime                  TEXT NOT NULL,
                                       wake_time_actual         TEXT NOT NULL,
                                       wake_time_target         TEXT NOT NULL,
                                       notes                    TEXT,
                                       nap_minutes              INTEGER NOT NULL,
                                       sleep_quality_score      INTEGER NOT NULL,
                                       total_sleep_minutes      INTEGER NOT NULL,
                                       awake_minutes            INTEGER NOT NULL,
                                       sleep_latency_minutes    INTEGER NOT NULL,
                                       wake_count               INTEGER NOT NULL,
                                       efficiency_actual_pct    REAL NOT NULL,
                                       efficiency_vs_target_pct REAL NOT NULL
);
