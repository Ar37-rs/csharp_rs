using System;
using System.Collections.Generic;

namespace csharp_rs
{

    class Program
    {
        public static void Main(string[] args)
        {

            // send a line of string to Rust then receive List<double> from Rust

            string csstring = "here i'm from dotnet! 👀";

            string rsstring = Rust.rsString(csstring);

            List<double> list_csdouble = Rust.listDouble(rsstring);

            for (int i = 0; i < list_csdouble.Count; ++i)
            {
                Console.WriteLine(list_csdouble[i]);
            }
        }
    }
}
