package com.example.fsrs;

public class Card {
    private static native long New();

    private static native long RecordLogGet(long card, long rating);

    private static native String ScheduledtoString(long card);

    private static native long ScheduledDays(long card);

    private static native void ScheduledSetDays(long card, long days);

    private static native long Due(long card);

    private static native void SetDue(long card, long due_timestamp);

    private static native double Stability(long card);

    private static native void SetStability(long card, double stability);

    private static native double Difficulty(long card);

    private static native void SetDifficulty(long card, double difficulty);

    private static native long ElapsedDays(long card);

    private static native void SetElapsedDays(long card, long elapsed_days);

    private static native int Reps(long card);

    private static native void SetReps(long card, int reps);

    private static native int Lapses(long card);

    private static native void SetLapses(long card, int lapses);

    private static native int State(long card);

    private static native void SetState(long card, int state);

    private static native long LastReview(long card);

    private static native void SetLastReview(long card, long last_review_timestamp);

    private final long nativePtr;

    public Card() {
        this.nativePtr = New();
    }

    public Card(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public SchedulingInfo recordLog(long rating) {
        return new SchedulingInfo(RecordLogGet(nativePtr, rating));
    }

    public long toNative() {
        return nativePtr;
    }

    public SchedulingInfo get(long rating) {
        return new SchedulingInfo(RecordLogGet(nativePtr, rating));
    }

    @Override
    public String toString() {
        return ScheduledtoString(nativePtr);
    }

    public long getScheduledDays() {
        return ScheduledDays(nativePtr);
    }

    public void setScheduledDays(long days) {
        ScheduledSetDays(nativePtr, days);
    }

    public long getDue() {
        return Due(nativePtr);
    }

    public void setDue(long due_timestamp) {
        SetDue(nativePtr, due_timestamp);
    }

    public double getStability() {
        return Stability(nativePtr);
    }

    public void setStability(double stability) {
        SetStability(nativePtr, stability);
    }

    public double getDifficulty() {
        return Difficulty(nativePtr);
    }

    public void setDifficulty(double difficulty) {
        SetDifficulty(nativePtr, difficulty);
    }

    public long getElapsedDays() {
        return ElapsedDays(nativePtr);
    }

    public void setElapsedDays(long elapsed_days) {
        SetElapsedDays(nativePtr, elapsed_days);
    }

    public int getReps() {
        return Reps(nativePtr);
    }

    public void setReps(int reps) {
        SetReps(nativePtr, reps);
    }

    public int getLapses() {
        return Lapses(nativePtr);
    }

    public void setLapses(int lapses) {
        SetLapses(nativePtr, lapses);
    }

    public int getState() {
        return State(nativePtr);
    }

    public void setState(int state) {
        SetState(nativePtr, state);
    }

    public long getLastReview() {
        return LastReview(nativePtr);
    }

    public void setLastReview(long last_review_timestamp) {
        SetLastReview(nativePtr, last_review_timestamp);
    }

}
