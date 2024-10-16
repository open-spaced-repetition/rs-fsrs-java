package com.example.fsrs;

import java.time.Instant;

public class FSRS {
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
        long fsrs = FsrsDefault(gf);
        long card = CardNew(gf);
        long scheduledCard = repeat(gf, fsrs, card, Instant.now().getEpochSecond());
        for (long rating : new long[]{1,2,3,4}) {
            long scheduling_info = RecordLogGet(gf, scheduledCard, rating);
            System.out.println(getCardScheduledDays(gf,
                     SchedulingInfoCard(gf, scheduling_info)));
    
        }
     }
}
