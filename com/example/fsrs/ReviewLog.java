package com.example.fsrs;

public class ReviewLog {
    private final long nativePtr;

    private static native String toString(long ReviewLog);

    private static native int Rating(long reviewLog);

    private static native void SetRating(long reviewLog, int rating);

    private static native long ElapsedDays(long reviewLog);

    private static native void SetElapsedDays(long reviewLog, long elapsed_days);

    private static native long ScheduledDays(long reviewLog);

    private static native void SetScheduledDays(long reviewLog, long scheduled_days);

    private static native int State(long reviewLog);

    private static native void SetState(long reviewLog, int state);

    private static native long ReviewedDate(long reviewLog);

    private static native void SetReviewedDate(long reviewLog, long reviewed_date_timestamp);



    public ReviewLog(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    @Override
    public String toString() {
        return toString(nativePtr);
    }

    public int getRating() {
        return Rating(nativePtr);
    }

    public void setRating(int rating) {
        SetRating(nativePtr, rating);
    }

    public long getElapsedDays() {
        return ElapsedDays(nativePtr);
    }

    public void setElapsedDays(long elapsed_days) {
        SetElapsedDays(nativePtr, elapsed_days);
    }

    public long getScheduledDays() {
        return ScheduledDays(nativePtr);
    }

    public void setScheduledDays(long scheduled_days) {
        SetScheduledDays(nativePtr, scheduled_days);
    }

    public int getState() {
        return State(nativePtr);
    }

    public void setState(int state) {
        SetState(nativePtr, state);
    }

    public long getReviewedDate() {
        return ReviewedDate(nativePtr);
    }

    public void setReviewedDate(long reviewed_date_timestamp) {
        SetReviewedDate(nativePtr, reviewed_date_timestamp);
    }
}
