package com.example.fsrs;

import java.time.Instant;

public class FSRS {
    private static native long FsrsNew(FSRS callback, long parameter);

    private static native long ParameterNew(FSRS callback,
            int maximum_interval,
            double request_retention,
            double w1,
            double w2,
            double w3,
            double w4,
            double w5,
            double w6,
            double w7,
            double w8,
            double w9,
            double w10,
            double w11,
            double w12,
            double w13,
            double w14,
            double w15,
            double w16,
            double w17,
            double w18,
            double w19,
            double decay,
            double factor,
            boolean enable_short_term);

    private static native long FsrsDefault(FSRS callback);

    private static native long CardNew(FSRS callback);

    private static native long repeat(FSRS callback, long fsrs, long card, long second);

    private static native long RecordLogGet(FSRS callback, long card, long rating);

    private static native long getCardScheduledDays(FSRS callback, long card);

    private static native long SchedulingInfoCard(FSRS callback, long SchedulingInfo);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    public static void main(String[] args) {
        FSRS gf = new FSRS();
        long fsrs_ = FsrsNew(gf, ParameterNew(gf, 90, 0.9,
                0.5, 0.5, 0.5, 0.5, 0.5, //
                0.5, 0.5, 0.5, 0.5, 0.5, //
                0.5, 0.5, 0.5, 0.5, 0.5, //
                0.5, 0.5, 0.5, 0.5,
                0.9, 0.9, false));
        long fsrs = FsrsDefault(gf);
        long card = CardNew(gf);
        long scheduledCard = repeat(gf, fsrs, card, Instant.now().getEpochSecond());
        for (long rating : new long[] { 1, 2, 3, 4 }) {
            long scheduling_info = RecordLogGet(gf, scheduledCard, rating);
            System.out.println(getCardScheduledDays(gf,
                    SchedulingInfoCard(gf, scheduling_info)));

        }
    }
}
