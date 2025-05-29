package com.nathan.sleeptracker.services;

import com.nathan.sleeptracker.entities.Answer;
import com.nathan.sleeptracker.repositories.AnswerRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class AnswerService {
    private final AnswerRepository answerRepository;

    @Autowired
    public AnswerService(AnswerRepository answerRepository) {
        this.answerRepository = answerRepository;
    }

    public Answer saveAnswer(Answer answer) {
        // Calculate efficiency metrics before saving
        answer.setEfficiencyActualPct(calculateEfficiency(
                answer.getTotalSleepMinutes(),
                answer.getAwakeMinutes(),
                answer.getSleepLatencyMinutes()
        ));

        answer.setEfficiencyVsTargetPct(compareToTarget(
                answer.getBedtime(),
                answer.getWakeTimeTarget(),
                answer.getTotalSleepMinutes()
        ));

        return answerRepository.save(answer);
    }

    /**
     * Computes actual sleep efficiency as:
     *   (time actually asleep) ÷ (total time in bed) × 100
     * where total time in bed = slept + awake + latency.
     *
     * @param totalSleep    minutes actually asleep
     * @param awakeMinutes  minutes awake during the night
     * @param latency       minutes it took to fall asleep
     * @return efficiency percentage (rounded to 2 significant figures)
     */
    public double calculateEfficiency(int totalSleep, int awakeMinutes, int latency) {
        double timeInBed = totalSleep + awakeMinutes + latency;
        if (timeInBed == 0) return 0;
        return roundTo2SigFigs((double) totalSleep / timeInBed * 100);
    }

    /**
     * Compares actual sleep to the target sleep window:
     *   (actual sleep) ÷ (planned time in bed) × 100.
     * Planned time in bed is computed from bedtime to targetWakeTime.
     *
     * @param bedtime         HH:MM when you went to bed
     * @param targetWakeTime  HH:MM when you planned to wake
     * @param totalSleep      minutes actually asleep
     * @return efficiency vs. target (%) (rounded to 2 significant figures)
     */
    public double compareToTarget(String bedtime, String targetWakeTime, int totalSleep) {
        int targetSleepWindow = calculateTargetSleepWindowMinutes(bedtime, targetWakeTime);
        if (targetSleepWindow == 0) return 0;
        return roundTo2SigFigs((double) totalSleep / targetSleepWindow * 100);
    }

    /**
     * Calculates the target sleep window in minutes between bedtime and wake time.
     * Handles cases where wake time is past midnight.
     *
     * @param bedtime HH:MM format
     * @param targetWakeTime HH:MM format
     * @return duration in minutes
     */
    private int calculateTargetSleepWindowMinutes(String bedtime, String targetWakeTime) {
        try {
            // Parse HH:MM
            String[] bedtimeParts = bedtime.split(":");
            String[] wakeTimeParts = targetWakeTime.split(":");

            int bedHour = Integer.parseInt(bedtimeParts[0]);
            int bedMinute = Integer.parseInt(bedtimeParts[1]);
            int wakeHour = Integer.parseInt(wakeTimeParts[0]);
            int wakeMinute = Integer.parseInt(wakeTimeParts[1]);

            // Convert to minutes since midnight
            int bedtimeMinutes = bedHour * 60 + bedMinute;
            int wakeTimeMinutes = wakeHour * 60 + wakeMinute;

            // If wake time is earlier in the day than bedtime, assume next day
            if (wakeTimeMinutes <= bedtimeMinutes) {
                wakeTimeMinutes += 24 * 60; // Add 24 hours
            }

            return wakeTimeMinutes - bedtimeMinutes;
        } catch (Exception e) {
            // Handle parsing errors gracefully
            return 0;
        }
    }

    /**
     * Rounds a value to 2 significant figures.
     *
     * @param value the value to round
     * @return rounded value
     */
    private double roundTo2SigFigs(double value) {
        if (value == 0) return 0;
        final int scale = (int) Math.floor(Math.log10(Math.abs(value)));
        return Math.round(value * Math.pow(10, 1 - scale)) / Math.pow(10, 1 - scale);
    }
}