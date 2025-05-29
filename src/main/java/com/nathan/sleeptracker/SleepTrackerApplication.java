package com.nathan.sleeptracker;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class SleepTrackerApplication {

    public static void main(String[] args) {
        var context = SpringApplication.run(SleepTrackerApplication.class, args);
        var questionnaire = context.getBean(Questionnaire.class);
        questionnaire.run();

    }

}
