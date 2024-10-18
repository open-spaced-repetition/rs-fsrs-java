package com.example.fsrs;

import java.time.Instant;

public class FSRS {
    private static native long FsrsNew(long parameter);

    private static native long ParameterNew(
            int maximum_interval,
            double request_retention,
            double[] w,
            double decay,
            double factor,
            boolean enable_short_term);

    private static native long FsrsDefault();

    private static native long CardNew();

    private static native long FsrsRepeat(long fsrs, long card, long second);

    private static native long RecordLogGet(long card, long rating);

    private static native String CardScheduledtoString(long card);

    private static native long SchedulingInfoCard(long SchedulingInfo);

    private static native long SchedulingInfoReviewLog(long SchedulingInfo);

    private static native String ReviewLogtoString(long ReviewLog);

    private static native long CardScheduledDays(long card);

    private static native void CardScheduledSetDays(long card, long days);

    private static native long CardDue(long card);

    private static native void CardSetDue(long card, long due_timestamp);

    private static native double CardStability(long card);

    private static native void CardSetStability(long card, double stability);

    private static native double CardDifficulty(long card);

    private static native void CardSetDifficulty(long card, double difficulty);

    private static native long CardElapsedDays(long card);

    private static native void CardSetElapsedDays(long card, long elapsed_days);

    private static native int CardReps(long card);

    private static native void CardSetReps(long card, int reps);

    private static native int CardLapses(long card);

    private static native void CardSetLapses(long card, int lapses);

    private static native int CardState(long card);

    private static native void CardSetState(long card, int state);

    private static native long CardLastReview(long card);

    private static native void CardSetLastReview(long card, long last_review_timestamp);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    public static class Parameter {
        private final int maximumInterval;
        private final double requestRetention;
        private final double[] w;
        private final double decay;
        private final double factor;
        private final boolean enableShortTerm;

        public Parameter(int maximumInterval, double requestRetention, double[] w, double decay, double factor,
                boolean enableShortTerm) {
            this.maximumInterval = maximumInterval;
            this.requestRetention = requestRetention;
            this.w = w;
            this.decay = decay;
            this.factor = factor;
            this.enableShortTerm = enableShortTerm;
        }

        private long toNative() {
            return ParameterNew(maximumInterval, requestRetention, w, decay, factor, enableShortTerm);
        }
    }

    private long fsrs;
    private Parameter parameter;

    public FSRS(Parameter parameter) {
        this.parameter = parameter;
        this.fsrs = FsrsNew(parameter.toNative());
    }

    public Card repeat(Card card, long second) {
        return new Card(FsrsRepeat(fsrs, card.toNative(), second));

    }

    public static class Card {
        private final long nativePtr;

        public Card() {
            this.nativePtr = CardNew();
        }

        private Card(long nativePtr) {
            this.nativePtr = nativePtr;
        }

        public SchedulingInfo recordLog(long rating) {
            return new SchedulingInfo(RecordLogGet(nativePtr, rating));
        }

        private long toNative() {
            return nativePtr;
        }

        public SchedulingInfo get(long rating) {
            return new SchedulingInfo(RecordLogGet(nativePtr, rating));
        }

        @Override
        public String toString() {
            return CardScheduledtoString(nativePtr);
        }

        public long getScheduledDays() {
            return CardScheduledDays(nativePtr);
        }

        public void setScheduledDays(long days) {
            CardScheduledSetDays(nativePtr, days);
        }

        public long getDue() {
            return CardDue(nativePtr);
        }

        public void setDue(long due_timestamp) {
            CardSetDue(nativePtr, due_timestamp);
        }

        public double getStability() {
            return CardStability(nativePtr);
        }

        public void setStability(double stability) {
            CardSetStability(nativePtr, stability);
        }

        public double getDifficulty() {
            return CardDifficulty(nativePtr);
        }

        public void setDifficulty(double difficulty) {
            CardSetDifficulty(nativePtr, difficulty);
        }

        public long getElapsedDays() {
            return CardElapsedDays(nativePtr);
        }

        public void setElapsedDays(long elapsed_days) {
            CardSetElapsedDays(nativePtr, elapsed_days);
        }

        public int getReps() {
            return CardReps(nativePtr);
        }

        public void setReps(int reps) {
            CardSetReps(nativePtr, reps);
        }

        public int getLapses() {
            return CardLapses(nativePtr);
        }

        public void setLapses(int lapses) {
            CardSetLapses(nativePtr, lapses);
        }

        public int getState() {
            return CardState(nativePtr);
        }

        public void setState(int state) {
            CardSetState(nativePtr, state);
        }

        public long getLastReview() {
            return CardLastReview(nativePtr);
        }

        public void setLastReview(long last_review_timestamp) {
            CardSetLastReview(nativePtr, last_review_timestamp);
        }

    }

    public static class SchedulingInfo {
        private final long nativePtr;

        private SchedulingInfo(long nativePtr) {
            this.nativePtr = nativePtr;
        }

        public Card getCard() {
            return new Card(SchedulingInfoCard(nativePtr));
        }

        public ReviewLog getReviewLog() {
            return new ReviewLog(SchedulingInfoReviewLog(nativePtr));
        }
    }

    public static class ReviewLog {
        private final long nativePtr;

        private ReviewLog(long nativePtr) {
            this.nativePtr = nativePtr;
        }

        @Override
        public String toString() {
            return ReviewLogtoString(nativePtr);
        }
    }

    public FSRS() {
        this.fsrs = FsrsDefault();
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
        Card scheduledCard = fsrs.repeat(card, Instant.now().getEpochSecond());
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
        }
    }
}
