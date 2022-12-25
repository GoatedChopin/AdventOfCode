import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;

public class Day6_Lanternfish {
    static File data = new File("C:\\Users\\colby\\IdeaProjects\\AdventOfCode21\\d6data1");
    static int numFish;
    static int[] fish;

    static {
        try(BufferedReader bufferedReader = new BufferedReader(new FileReader(data))) {
            String[] inputString = bufferedReader.readLine().split(",");
            fish = new int[inputString.length];
            for (int i = 0; i < inputString.length; i++) {
                fish[i] = Integer.parseInt(inputString[i].trim());
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static int[] cycleLanternfish(int[] currentFish) {
        int newPopulationSize = currentFish.length;
        for (int i: currentFish) {
            if (i == 0) newPopulationSize++;
        }
        int[] newFish = new int[newPopulationSize];
        for (int i = 0; i < newPopulationSize; i++) {
            if (i < currentFish.length) {
                if (currentFish[i] == 0) newFish[i] = 6;
                else newFish[i] = currentFish[i] - 1;
            }
            else newFish[i] = 8;
        }
        return newFish;
    }

    public static void main(String[] args) {
//        for(int i = 0; i < 80; i++) {
//            fish = cycleLanternfish(fish);
//            System.out.println("Day " + (i+1) + " is " + fish.length);
//        }
//        System.out.println(fish.length);

        LanternTracker lanternTracker = new LanternTracker(fish);
        for (int i = 0; i < 256; i++) {
            lanternTracker.cycleLanternFish();
            System.out.println(lanternTracker.returnTotalFish());
        }
    }

    public static class LanternTracker {
        long num0s;
        long num1s;
        long num2s;
        long num3s;
        long num4s;
        long num5s;
        long num6s;
        long num7s;
        long num8s;

        public void cycleLanternFish() {
            long temp = num0s;
            long temp2 = num8s;
            num8s = num0s;
            num0s = num1s;
            num1s = num2s;
            num2s = num3s;
            num3s = num4s;
            num4s = num5s;
            num5s = num6s;
            num6s = num7s + temp;
            num7s = temp2;
        }

        public long returnTotalFish() {
            long totalFish = num0s + num1s + num2s + num3s + num4s + num5s + num6s + num7s + num8s;
            return totalFish;
        }

        public LanternTracker(int[] currentFish) {
            for(int i: currentFish) {
                switch (i) {
                    case 0: num0s++;
                            break;
                    case 1: num1s++;
                            break;
                    case 2: num2s++;
                            break;
                    case 3: num3s++;
                            break;
                    case 4: num4s++;
                            break;
                    case 5: num5s++;
                            break;
                    case 6: num6s++;
                            break;
                    case 7: num7s++;
                            break;
                    case 8: num8s++;
                            break;
                }
            }
            System.out.println(num1s);
            System.out.println(num2s);
            System.out.println(num3s);
        }


    }
}
