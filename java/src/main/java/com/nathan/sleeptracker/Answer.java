package com.nathan.sleeptracker;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
public class Answer {
    private long   id;
    private String entryDate, bedtime, wakeTimeTarget, wakeTimeActual, notes;
    private int    napMinutes, sleepQualityScore, totalSleepMinutes,
            awakeMinutes, sleepLatencyMinutes, wakeCount;
}