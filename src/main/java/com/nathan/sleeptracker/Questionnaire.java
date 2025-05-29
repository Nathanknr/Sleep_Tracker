package com.nathan.sleeptracker;
import com.nathan.sleeptracker.services.AnswerService;
import com.nathan.sleeptracker.model.Answer;
import org.springframework.stereotype.Component;

import java.time.LocalDate;
import java.util.Scanner;

@Component
/**
 * Questionnaire for collecting sleep data and persisting via AnswerService.
 */
public class Questionnaire {
    private final AnswerService answerService;
    private final Scanner scanner;

    public Questionnaire(AnswerService answerService) {
        this.answerService = answerService;
        this.scanner = new Scanner(System.in);
    }

    /**
     * Runs the questionnaire, collects responses, builds an Answer, and saves it.
     */
    public void run() {
        System.out.println("--- Sleep Tracker Questionnaire ---");

        String entryDate = LocalDate.now().toString();
        String bedtime = prompt("Enter bedtime (HH:MM): ");
        String wakeTimeTarget = prompt("Enter target wake time (HH:MM): ");
        String wakeTimeActual = prompt("Enter actual wake time (HH:MM): ");
        int napMinutes = Integer.parseInt(prompt("Enter nap duration in minutes: "));
        int sleepQualityScore = Integer.parseInt(prompt("Enter sleep quality score (1-5): "));
        int totalSleepMinutes = Integer.parseInt(prompt("Enter total sleep duration in minutes: "));
        int awakeMinutes = Integer.parseInt(prompt("Enter minutes awake during the night: "));
        int sleepLatencyMinutes = Integer.parseInt(prompt("Enter time to fall asleep in minutes: "));
        int wakeCount = Integer.parseInt(prompt("Enter number of awakenings: "));
        String notes = prompt("Any additional notes? (press Enter to skip): ");

        // Calculate efficiencies
        int efficiencyActualPct = answerService.calculateEfficiency(totalSleepMinutes, awakeMinutes, sleepLatencyMinutes);
        int efficiencyVsTargetPct = answerService.compareToTarget(bedtime, wakeTimeTarget, totalSleepMinutes);;

        // Build and save Answer
        Answer answer = Answer.builder()
                .entryDate(entryDate)
                .bedtime(bedtime)
                .wakeTimeTarget(wakeTimeTarget)
                .wakeTimeActual(wakeTimeActual)
                .napMinutes(napMinutes)
                .sleepQualityScore(sleepQualityScore)
                .totalSleepMinutes(totalSleepMinutes)
                .awakeMinutes(awakeMinutes)
                .sleepLatencyMinutes(sleepLatencyMinutes)
                .wakeCount(wakeCount)
                .notes(notes)
                .efficiencyActualPct(efficiencyActualPct)
                .efficiencyVsTargetPct(efficiencyVsTargetPct)
                .build();

        answerService.save(answer);
        System.out.println("Entry saved successfully.");
    }

    private String prompt(String message) {
        System.out.print(message);
        return scanner.nextLine().trim();
    }
}
