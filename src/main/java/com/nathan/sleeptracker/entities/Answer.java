package com.nathan.sleeptracker.entities;

import jakarta.persistence.*;
import lombok.*;

/**
 * JPA entity representing a sleep questionnaire entry.
 */
@Entity
@Table(name = "answers")
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class Answer{


    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(name = "entry_date", nullable = false)
    private String entryDate;         // YYYY-MM-DD

    @Column(name = "bedtime", nullable = false)
    private String bedtime;           // HH:MM of when you went to bed

    @Column(name = "wake_time_actual", nullable = false)
    private String wakeTimeActual;    // HH:MM of when you actually woke up

    @Column(name = "wake_time_target", nullable = false)
    private String wakeTimeTarget;    // HH:MM of planned wake time

    @Column(name = "notes")
    private String notes;             // free-form comments

    @Column(name = "nap_minutes", nullable = false)
    private int napMinutes;           // nap duration in minutes

    @Column(name = "sleep_quality_score", nullable = false)
    private int sleepQualityScore;    // e.g. 1–5 scale

    @Column(name = "total_sleep_minutes", nullable = false)
    private int totalSleepMinutes;    // total sleep time in minutes

    @Column(name = "awake_minutes", nullable = false)
    private int awakeMinutes;         // minutes awake during the night

    @Column(name = "sleep_latency_minutes", nullable = false)
    private int sleepLatencyMinutes;  // minutes to fall asleep

    @Column(name = "wake_count", nullable = false)
    private int wakeCount;            // number of awakenings

    @Column(name = "efficiency_actual_pct")
    private Integer efficiencyActualPct;

    @Column(name = "efficiency_vs_target_pct")
    private Integer efficiencyVsTargetPct;

}
