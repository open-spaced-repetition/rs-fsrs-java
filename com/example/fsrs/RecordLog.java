package com.example.fsrs;

public class RecordLog {
    private final long nativePtr;
    RecordLog(long ptr) {
        nativePtr = ptr;
    }
    private static native long SchedulingInfo(long card, long rating);
    public SchedulingInfo get(long rating) {
        return new SchedulingInfo(SchedulingInfo(nativePtr, rating));
    }
}
