package com.nathan.sleeptracker.services;

import com.nathan.sleeptracker.repositories.AnswerRepository;
import org.springframework.stereotype.Service;

@Service
 public class AnswerService {
    private final AnswerRepository answerRepository;j

    public void saveAnswer(answer) {
       answerRepository.save(answer);

    }
        /**
         * Computes actual sleep efficiency as:
         *   (time actually asleep) ÷ (total time in bed) × 100
         * where total time in bed = slept + awake + latency.
         *
         * @param totalSleep    minutes actually asleep
         * @param awakeMinutes  minutes awake during the night
         * @param latency       minutes it took to fall asleep
         * @return efficiency percentage (rounded)
         */
        public int calculateEfficiency(int totalSleep, int awakeMinutes, int latency) {
            int timeInBed = totalSleep + awakeMinutes + latency;
            if (timeInBed == 0) {
                return 0;
            }
            double eff = (100.0 * totalSleep) / timeInBed;
            return (int) Math.round(eff);
        }

        /**
         * Compares actual sleep to the target sleep window:
         *   (actual sleep) ÷ (planned time in bed) × 100.
         * Planned time in bed is computed from bedtime to targetWakeTime.
         *
         * @param bedtime         HH:MM when you went to bed
         * @param targetWakeTime  HH:MM when you planned to wake
         * @param totalSleep      minutes actually asleep
         * @return efficiency vs. target (%) (rounded)
         */
        public int compareToTarget(String bedtime, String targetWakeTime, int totalSleep) {
            // parse HH:MM
            String[] in = bedtime.split(":");
            String[] out = targetWakeTime.split(":");
            int inH = Integer.parseInt(in[0]);
            int inM = Integer.parseInt(in[1]);
            int outH = Integer.parseInt(out[0]);
            int outM = Integer.parseInt(out[1]);

            // compute minutes since midnight
            int start = inH * 60 + inM;
            int end = outH * 60 + outM;
            // if target is past midnight
            if (end <= start) {
                end += 24 * 60;
            }
            int plannedDuration = end - start;
            if (plannedDuration == 0) {
                return 0;
            }
            double pct = (100.0 * totalSleep) / plannedDuration;
            return (int) Math.round(pct);
        }
    }


