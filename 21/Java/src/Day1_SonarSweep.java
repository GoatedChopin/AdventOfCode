import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Scanner;

public class Day1_SonarSweep {
    static File dataFile = new File("C:\\Users\\Colby\\Programming\\Java\\AdventOfCode\\d1data1");
    static ArrayList<Integer> partTwoArrayList = new ArrayList<>();

    public static int findOneLengthIncreases(File file) {
        try(Scanner scanner = new Scanner(new FileReader(file))) {
            int numGreater = 0;
            int numLines = 0;
            int currentDepth = Integer.parseInt(scanner.nextLine());
            while (scanner.hasNext()) {
                int nextDepth = Integer.parseInt(scanner.nextLine());
                if (nextDepth > currentDepth) numGreater++;
                currentDepth = nextDepth;
                numLines++;
            }
            System.out.println(numGreater + " depths were increases out of a total of " + numLines + " observations.");
            return numGreater;
        } catch (FileNotFoundException e) {
            return -1;
        }
    }

    public static int findThreeLengthIncreases(File file) {
        try(Scanner scanner = new Scanner(new FileReader(file))) {
            while(scanner.hasNext()) partTwoArrayList.add(Integer.parseInt(scanner.nextLine()));
            int numGreater = 0;
            int currentWindow = 0;
            int nextWindow = 0;
            for (int i = 0; i < partTwoArrayList.size() - 3; i++) {
                currentWindow = partTwoArrayList.get(i) + partTwoArrayList.get(i+1) + partTwoArrayList.get(i+2);
                nextWindow = partTwoArrayList.get(i+1) + partTwoArrayList.get(i+2) + partTwoArrayList.get(i+3);
                if (currentWindow < nextWindow) numGreater++;
            }
            System.out.println(numGreater + " depth windows were increases out of a total of " + partTwoArrayList.size() + " observations.");
            return numGreater;
        } catch (FileNotFoundException e) {
            return -1;
        }
    }

    public static void main(String[] args) {
        int answer = findOneLengthIncreases(dataFile);
        System.out.println(answer);
        answer = findThreeLengthIncreases(dataFile);
        System.out.println(answer);
    }
}
