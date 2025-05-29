package com.nathan.sleeptracker;

import com.nathan.sleeptracker.entities.Answer;
import com.nathan.sleeptracker.services.AnswerService;
import org.springframework.stereotype.Component;

import java.time.LocalDate;
import java.util.Scanner;

/**
 * Questionnaire for collecting sleep data and persisting via AnswerService.
 */
@Component
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

        try {
            String entryDate = LocalDate.now().toString();
            String bedtime = prompt("Enter bedtime (HH:MM): ");
            String wakeTimeTarget = prompt("Enter target wake time (HH:MM): ");
            String wakeTimeActual = prompt("Enter actual wake time (HH:MM): ");
            int napMinutes = parseInt(prompt("Enter nap duration in minutes: "));
            int sleepQualityScore = parseInt(prompt("Enter sleep quality score (1-5): "));
            int totalSleepMinutes = parseInt(prompt("Enter total sleep duration in minutes: "));
            int awakeMinutes = parseInt(prompt("Enter minutes awake during the night: "));
            int sleepLatencyMinutes = parseInt(prompt("Enter time to fall asleep in minutes: "));
            int wakeCount = parseInt(prompt("Enter number of awakenings: "));
            String notes = prompt("Any additional notes? (press Enter to skip): ");

            // Build Answer (efficiency calculations will be done in service layer)
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
                    .build();

            Answer savedAnswer = answerService.saveAnswer(answer);
            System.out.println("Entry saved successfully with ID: " + savedAnswer.getId());
            System.out.printf("Sleep Efficiency: %.2f%%\n", savedAnswer.getEfficiencyActualPct());
            System.out.printf("Efficiency vs Target: %.2f%%\n", savedAnswer.getEfficiencyVsTargetPct());

        } catch (Exception e) {
            System.err.println("Error processing questionnaire: " + e.getMessage());
        }
    }

    private String prompt(String message) {
        System.out.print(message);
        return scanner.nextLine().trim();
    }

    private int parseInt(String input) {
        try {
            return Integer.parseInt(input);
        } catch (NumberFormatException e) {
            System.out.println("Invalid number format, using 0 as default.");
            return 0;
        }
    }
}