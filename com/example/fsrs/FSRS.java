package com.example.fsrs;

import java.time.Instant;

public class FSRS {
    private static native long FsrsNew(FSRS callback, long parameter);

    private static native long ParameterNew(FSRS callback,
            int maximum_interval,
            double request_retention,
            double[] w,
            double decay,
            double factor,
            boolean enable_short_term);

    private static native long FsrsDefault(FSRS callback);

    private static native long CardNew(FSRS callback);

    private static native long repeat(FSRS callback, long fsrs, long card, long second);

    private static native long RecordLogGet(FSRS callback, long card, long rating);

    private static native long CardScheduledDays(FSRS callback, long card);

    private static native String CardScheduledtoString(FSRS callback, long card);

    private static native long SchedulingInfoCard(FSRS callback, long SchedulingInfo);

    private static native long SchedulingInfoReviewLog(FSRS callback, long SchedulingInfo);

    private static native String ReviewLogtoString(FSRS callback, long ReviewLog);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    public static void main(String[] args) {
        FSRS gf = new FSRS();
        long fsrs_ = FsrsNew(gf, ParameterNew(gf, 90, 0.9,
                new double[] {
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5, 0.5,
                        0.5, 0.5, 0.5, 0.5 },
                0.9, 0.9, false));
        long fsrs = FsrsDefault(gf);
        long card = CardNew(gf);
        long scheduledCard = repeat(gf, fsrs, card, Instant.now().getEpochSecond());
        for (long rating : new long[] { 1, 2, 3, 4 }) {
            long scheduling_info = RecordLogGet(gf, scheduledCard, rating);
            long card_ = SchedulingInfoCard(gf, scheduling_info);
            long review_log = SchedulingInfoReviewLog(gf, scheduling_info);
            System.out.println(CardScheduledtoString(gf, card_));
            System.out.println(ReviewLogtoString(gf, review_log));
        }
    }
}
