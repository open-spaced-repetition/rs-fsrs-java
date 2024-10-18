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

    private static native long CardScheduledDays(long card);

    private static native String CardScheduledtoString(long card);

    private static native long SchedulingInfoCard(long SchedulingInfo);

    private static native long SchedulingInfoReviewLog(long SchedulingInfo);

    private static native String ReviewLogtoString(long ReviewLog);

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
            System.out.println(review_log.toString());
        }
    }
}
