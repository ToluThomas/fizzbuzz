// To run
// mcs Cs/cs.cs
// mono cs.exe

using System;

namespace Cs {
    class Cs {
        static void Main(String[] args) {
            int n = Int32.Parse(args[0]);
            char[] f = { 'F', 'i', 'z', 'z' };
            char[] b = { 'B', 'u', 'z', 'z' };
            char[] fb = { 'F', 'i', 'z', 'z', 'B', 'u', 'z', 'z' };

            for (int i = 1; i < n + 1; i++) {
                if ((i % 3 == 0) && (i % 5 == 0)) {
                    Console.WriteLine(fb);
                } else if (i % 3 == 0) {
                    Console.WriteLine(f);
                } else if (i % 5 == 0) {
                    Console.WriteLine(b);
                } else {
                    Console.WriteLine(i);
                }
            }
        }
    }
}