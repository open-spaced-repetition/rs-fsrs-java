package com.example.fsrs;

import java.time.Instant;

public class FSRS {
    private static native long FsrsNew(FSRS callback);

    private static native long CardNew(FSRS callback);

    private static native long schedule(FSRS callback, long fsrs, long card, long second);
    private static native long selectCard(FSRS callback, long card, long rating);
    private static native long getCardScheduledDays(long card);

    static {
        System.loadLibrary("rs_fsrs_java");
    }

    public static void main(String[] args) {
        FSRS gf = new FSRS();
        long f = FsrsNew(gf);
        long c = CardNew(gf);
        long scheduledCard = schedule(gf, f, c, Instant.now().getEpochSecond());
        long updated_card = selectCard(gf, scheduledCard, 4);
        System.out.println(getCardScheduledDays(updated_card));
    }
}
