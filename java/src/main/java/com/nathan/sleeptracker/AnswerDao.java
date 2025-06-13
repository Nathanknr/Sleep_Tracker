package com.nathan.sleeptracker;

import org.jdbi.v3.sqlobject.config.RegisterBeanMapper;
import org.jdbi.v3.sqlobject.customizer.Bind;
import org.jdbi.v3.sqlobject.statement.*;

import java.util.List;

public interface AnswerDao {
    @SqlUpdate("""
    CREATE TABLE IF NOT EXISTS answer (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      entry_date TEXT, bedtime TEXT, wake_time_target TEXT, wake_time_actual TEXT,
      nap_minutes INTEGER, sleep_quality_score INTEGER, total_sleep_minutes INTEGER,
      awake_minutes INTEGER, sleep_latency_minutes INTEGER, wake_count INTEGER,
      notes TEXT
    )
  """)
    void createTable();

    @SqlUpdate("""
    INSERT INTO answer (
      entry_date, bedtime, wake_time_target, wake_time_actual,
      nap_minutes, sleep_quality_score, total_sleep_minutes,
      awake_minutes, sleep_latency_minutes, wake_count, notes
    ) VALUES (
      :entryDate, :bedtime, :wakeTarget, :wakeActual,
      :nap, :quality, :total, :awake, :latency, :count, :notes
    )
  """)
    @GetGeneratedKeys
    long insert(
            @Bind("entryDate") String entryDate,
            @Bind("bedtime") String bedtime,
            @Bind("wakeTarget") String wakeTarget,
            @Bind("wakeActual") String wakeActual,
            @Bind("nap") int nap,
            @Bind("quality") int quality,
            @Bind("total") int total,
            @Bind("awake") int awake,
            @Bind("latency") int latency,
            @Bind("count") int count,
            @Bind("notes") String notes
    );

    @SqlQuery("SELECT * FROM answer ORDER BY id DESC")
    @RegisterBeanMapper(Answer.class)
    List<Answer> listAll();
}
