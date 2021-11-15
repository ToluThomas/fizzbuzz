package Java;

import java.lang.System;

public class Java {
    public static void main(String[] args) {
        int n = Integer.parseInt(args[0]);
        char[] f = { 'F', 'i', 'z', 'z' };
        char[] b = { 'B', 'u', 'z', 'z' };
        char[] fb = { 'F', 'i', 'z', 'z', 'B', 'u', 'z', 'z' };

        for (int i = 1; i < n + 1; i++) {
            if ((i % 3 == 0) && (i % 5 == 0)) {
                System.out.println(fb);
            } else if (i % 3 == 0) {
                System.out.println(f);
            } else if (i % 5 == 0) {
                System.out.println(b);
            } else {
                System.out.println(String.valueOf(i));
            }
        }
    }
}