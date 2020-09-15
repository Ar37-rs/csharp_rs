using System;
using System.Runtime.InteropServices;
using System.Collections.Generic;

namespace csharp_rs
{
    struct Rust
    {
        [DllImport("libcsharp_rs")]
        static extern IntPtr rs_entry_point(IntPtr ptr);

        public static string rsString(string csstring)
        {
            IntPtr rs_ptr = rs_entry_point(Marshal.StringToCoTaskMemUTF8(csstring));
            string rsstring = Marshal.PtrToStringUTF8(rs_ptr);
            // free the pointer here, used already.
            Marshal.FreeCoTaskMem(rs_ptr);
            return rsstring;
        }

        public static List<double> listDouble(string rsstring) {
            List<double> list_double = new List<double>();
            foreach(string item in rsstring.Split(char.Parse("`")))
            {
                double f64;
                if(double.TryParse(item, out f64))
                {
                    list_double.Add(f64);
                }
            }
            return list_double;
        }
    }
}