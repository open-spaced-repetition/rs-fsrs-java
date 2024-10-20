package com.example.fsrs;

public class SchedulingInfo {
    private final long nativePtr;

    private static native long Card(long SchedulingInfo);

    private static native long ReviewLog(long SchedulingInfo);

    public SchedulingInfo(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public Card getCard() {
        return new Card(Card(nativePtr));
    }

    public ReviewLog getReviewLog() {
        return new ReviewLog(ReviewLog(nativePtr));
    }
}
