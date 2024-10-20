package com.example.fsrs;

import java.time.Instant;

public class FSRS {
    private static native long New(long parameter);

    private static native long Default();

    private static native long Repeat(long fsrs, long card, long second);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    private long fsrs;
    private Parameter parameter;

    public FSRS(Parameter parameter) {
        this.parameter = parameter;
        this.fsrs = New(parameter.toNative());
    }

    public RecordLog repeat(Card card, Instant now) {
        return new RecordLog(Repeat(fsrs, card.toNative(), now.getEpochSecond()));
    }

    public FSRS() {
        this.fsrs = Default();
    }

    public static void main(String[] args) {
        FSRS fsrs = new FSRS(new Parameter(90, 0.9,
                new double[] {
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5 },
                0.9, 0.9, false));
        fsrs = new FSRS();
        Card card = new Card();
        RecordLog scheduledCard = fsrs.repeat(card, Instant.now());
        for (long rating : new long[] { 1, 2, 3, 4 }) {
            SchedulingInfo scheduling_info = scheduledCard.get(rating);
            card = scheduling_info.getCard();
            ReviewLog review_log = scheduling_info.getReviewLog();
            System.out.println(card.toString());
            System.out.println("Card Details:");
            System.out.println("Scheduled Days: " + card.getScheduledDays());
            System.out.println("Due Timestamp: " + card.getDue());
            System.out.println("Stability: " + card.getStability());
            System.out.println("Difficulty: " + card.getDifficulty());
            System.out.println("Elapsed Days: " + card.getElapsedDays());
            System.out.println("Reps: " + card.getReps());
            System.out.println("Lapses: " + card.getLapses());
            System.out.println("State: " + card.getState());
            System.out.println("Last Review Timestamp: " + card.getLastReview());
            System.out.println(review_log.toString());

            // Print ReviewLog fields using getters
            System.out.println("ReviewLog Details:");
            System.out.println("Rating: " + review_log.getRating());
            System.out.println("Elapsed Days: " + review_log.getElapsedDays());
            System.out.println("Scheduled Days: " + review_log.getScheduledDays());
            System.out.println("State: " + review_log.getState());
            System.out.println("Reviewed Date Timestamp: " + review_log.getReviewedDate());
        }
    }
}
