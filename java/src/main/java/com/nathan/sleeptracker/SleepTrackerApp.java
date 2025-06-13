package com.nathan.sleeptracker;

import org.jdbi.v3.core.Jdbi;
import org.jdbi.v3.sqlobject.SqlObjectPlugin;
import java.time.LocalDate;
import java.util.Scanner;

public class SleepTrackerApp {
    public static void main(String[] args) {
        Jdbi jdbi = Jdbi.create("jdbc:sqlite:tracker.sqlite");
        jdbi.installPlugin(new SqlObjectPlugin()); // Add this line!

        AnswerDao dao = jdbi.onDemand(AnswerDao.class);
        dao.createTable();

        Scanner in = new Scanner(System.in);
        System.out.println("--- Sleep Tracker ---");

        String entryDate = LocalDate.now().toString();
        System.out.print("Enter bedtime (HH:MM): ");     String b = in.nextLine();
        System.out.print("Enter target wake (HH:MM): "); String wt = in.nextLine();
        System.out.print("Enter actual wake (HH:MM): "); String wa = in.nextLine();
        System.out.print("Nap minutes: ");               int nap = Integer.parseInt(in.nextLine());
        System.out.print("Quality (1–5): ");             int q = Integer.parseInt(in.nextLine());
        System.out.print("Total sleep (HH:MM): ");       String tot = in.nextLine();
        System.out.print("Awake minutes: ");             int aw = Integer.parseInt(in.nextLine());
        System.out.print("Latency (min): ");             int lat = Integer.parseInt(in.nextLine());
        System.out.print("Wake count: ");                int wc = Integer.parseInt(in.nextLine());
        System.out.print("Notes: ");                     String notes = in.nextLine();

        int totalMin = toMinutes(tot);
        double eff = calcEff(totalMin, aw, lat);
        double vsTarget = 100.0 * totalMin / calcWindow(b, wt);

        long id = dao.insert(entryDate, b, wt, wa, nap, q, totalMin, aw, lat, wc, notes);
        System.out.printf("Saved ID=%d · Efficiency=%.1f%% · vsTarget=%.1f%%%n", id, eff, vsTarget);
    }

    static int toMinutes(String hhmm) {
        try {
            // Parse HH:MM
            String[] timeParts= hhmm.split(":");

            int hours = Integer.parseInt(timeParts[0]);
            int minutes = Integer.parseInt(timeParts[1]);

            // Convert to minutes since midnight
            return hours * 60 + minutes;

        } catch (Exception e) {
            return 0;
        }
    }

    static int calcWindow(String b, String wt) {
        try {
            // Parse HH:MM
            String[] bedtimeParts = b.split(":");
            String[] wakeTimeParts = wt.split(":");

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

    private static double roundTo2SigFigs(double value) {
        if (value == 0) return 0;
        final int scale = (int) Math.floor(Math.log10(Math.abs(value)));
        return Math.round(value * Math.pow(10, 1 - scale)) / Math.pow(10, 1 - scale);
    }

    static double calcEff(int sleep, int awake, int latency) {
        double timeInBed = sleep + awake + latency;
        if (timeInBed == 0) return 0;
        return roundTo2SigFigs((double) sleep / timeInBed * 100);
    }
}