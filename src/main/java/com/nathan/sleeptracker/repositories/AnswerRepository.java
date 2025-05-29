package com.nathan.sleeptracker.repositories;

import com.nathan.sleeptracker.entities.Answer;
import org.springframework.data.jpa.repository.JpaRepository;

public interface AnswerRepository extends JpaRepository<Answer, Long> {
}
