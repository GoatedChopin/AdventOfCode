import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class Day8_Septadecimal {

    static int num1;
    static int num4;
    static int num7;
    static int num8;
    static ArrayList<String> codes = new ArrayList<>();
    static File data = new File("d8data1");

    public static void main(String[] args) {
        // part 1
        int len;
        for (String s: codes) {
            len = s.length();
            switch (len) {
                case 2: num1++; break;
                case 4: num4++; break;
                case 3: num7++; break;
                case 7: num8++; break;
            }
        }
        System.out.println(num1+num4+num7+num8);
    }

    static {
        String line;
        try (BufferedReader bufferedReader = new BufferedReader(new FileReader(data))) {
            while ((line = bufferedReader.readLine()) != null) {
                String[] split1 = line.split("\\|");
                System.out.println(split1[1]);
                String[] split2 = split1[1].split(" ");
                for (String s: split2) {
                    codes.add(s);
//                    System.out.println(s);
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
