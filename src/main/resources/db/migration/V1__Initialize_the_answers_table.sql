CREATE TABLE answers_new (
                             id                       INTEGER PRIMARY KEY AUTOINCREMENT,
                             entry_date               TEXT    NOT NULL DEFAULT (date('now')),  -- date of this record
                             bedtime                  TEXT    NOT NULL,                       -- when you went to bed
                             wake_time_actual         TEXT    NOT NULL,                       -- when you finally woke up
                             wake_time_target         TEXT    NOT NULL,                       -- planned wake time
                             notes                    TEXT,                                   -- free-form comments
                             nap_minutes              INTEGER NOT NULL,                       -- nap duration (min)
                             sleep_quality_score      INTEGER NOT NULL,                       -- e.g. 1–5 scale
                             total_sleep_minutes      INTEGER NOT NULL,                       -- total sleep duration (min)
                             awake_minutes            INTEGER NOT NULL,                       -- minutes awake overnight
                             sleep_latency_minutes    INTEGER NOT NULL,                       -- minutes to fall asleep
                             wake_count               INTEGER NOT NULL,                       -- number of awakenings
                             efficiency_actual_pct    INTEGER,                                -- actual sleep efficiency (%)
                             efficiency_vs_target_pct INTEGER                                 -- actual vs. target efficiency (%)
);