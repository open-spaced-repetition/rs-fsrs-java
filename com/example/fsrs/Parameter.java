package com.example.fsrs;

public class Parameter {
        private final int maximumInterval;
        private final double requestRetention;
        private final double[] w;
        private final double decay;
        private final double factor;
        private final boolean enableShortTerm;
        private static native long New(
            int maximum_interval,
            double request_retention,
            double[] w,
            double decay,
            double factor,
            boolean enable_short_term);

        public Parameter(int maximumInterval, double requestRetention, double[] w, double decay, double factor,
                boolean enableShortTerm) {
            this.maximumInterval = maximumInterval;
            this.requestRetention = requestRetention;
            this.w = w;
            this.decay = decay;
            this.factor = factor;
            this.enableShortTerm = enableShortTerm;
        }

        public long toNative() {
            return New(maximumInterval, requestRetention, w, decay, factor, enableShortTerm);
        }
    
}
