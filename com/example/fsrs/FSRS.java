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

    private static native long repeat(long fsrs, long card, long second);

    private static native long RecordLogGet(long card, long rating);

    private static native long CardScheduledDays(long card);

    private static native String CardScheduledtoString(long card);

    private static native long SchedulingInfoCard(long SchedulingInfo);

    private static native long SchedulingInfoReviewLog(long SchedulingInfo);

    private static native String ReviewLogtoString(long ReviewLog);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    public static void main(String[] args) {
        long fsrs_ = FsrsNew(ParameterNew(90, 0.9,
                new double[] {
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5 },
                0.9, 0.9, false));
        long fsrs = FsrsDefault();
        long card = CardNew();
        long scheduledCard = repeat(fsrs, card, Instant.now().getEpochSecond());
        for (long rating : new long[] { 1, 2, 3, 4 }) {
            long scheduling_info = RecordLogGet(scheduledCard, rating);
            long card_ = SchedulingInfoCard(scheduling_info);
            long review_log = SchedulingInfoReviewLog(scheduling_info);
            System.out.println(CardScheduledtoString(card_));
            System.out.println(ReviewLogtoString(review_log));
        }
    }
}
