package com.eranm.users.enums;

public enum Status {
    STOPPED(9), FROZEN(99), ACTIVE(3);

    int statusCode;

    Status(int code) {
        this.statusCode = code;
    }

    static public Status getStatus(int code) {
        switch (code) {
            case 9:
                return STOPPED;
            case 99:
                return FROZEN;
            case 3:
                return ACTIVE;
            default:
                throw new IllegalArgumentException();
        }
    }
}

