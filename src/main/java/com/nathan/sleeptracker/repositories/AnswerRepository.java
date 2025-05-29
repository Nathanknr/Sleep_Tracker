package com.nathan.sleeptracker.repositories;

import com.nathan.sleeptracker.entities.Answer;
import org.springframework.data.jpa.repository.JpaRepository;

interface AnswerRepository extends JpaRepository<Answer, Long> {
}
