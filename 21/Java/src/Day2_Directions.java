import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Scanner;

public class Day2_Directions {
    static File dataFile = new File("C:\\Users\\Colby\\Programming\\Java\\AdventOfCode\\d2data1");
    static ArrayList<String> directionsList = new ArrayList<>();
    static ArrayList<Integer> distanceList = new ArrayList<>();

    static {
        try (Scanner scanner = new Scanner(new FileReader(dataFile))) {
            while (scanner.hasNext()) {
                String currentLine = scanner.nextLine();
                String[] currentArray = currentLine.split(" ");
                String direction = currentArray[0];
                Integer distance = Integer.parseInt(currentArray[1]);
                directionsList.add(direction);
                distanceList.add(distance);
            }
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
    }

    public static int[] followDirection1(int horizontalOrigin, int depthOrigin, String direction, int distance) {
        if (direction.equals("forward")) horizontalOrigin += distance;
        if (direction.equals("up")) depthOrigin -= distance;
        if (direction.equals("down")) depthOrigin += distance;
        int[] newPosition = {horizontalOrigin, depthOrigin};
        return newPosition;
    }

    public static int[] followDirection2(int horizontalOrigin, int depthOrigin, int aimOrigin, String direction, int distance) {
        if (direction.equals("forward")) {
            horizontalOrigin += distance;
            depthOrigin += aimOrigin * distance;
        }
        if (direction.equals("up")) aimOrigin -= distance;
        if (direction.equals("down")) aimOrigin += distance;
        int[] newPosition = {horizontalOrigin, depthOrigin, aimOrigin};
        return newPosition;
    }

    public static int plotCourse1() {
        int horizontal = 0;
        int depth = 0;
        for (int i = 0; i < directionsList.size(); i++) {
            int[] newPosition = followDirection1(horizontal, depth, directionsList.get(i), distanceList.get(i));
            horizontal = newPosition[0];
            depth = newPosition[1];
            System.out.println("Now at a depth of " + depth + " and a horizontal position of " + horizontal);
        }
        System.out.println(horizontal * depth);
        return horizontal * depth;
    }

    public static int plotCourse2() {
        int horizontal = 0;
        int depth = 0;
        int aim = 0;
        for (int i = 0; i < directionsList.size(); i++) {
            int[] currentPosition = followDirection2(horizontal, depth, aim, directionsList.get(i), distanceList.get(i));
            horizontal = currentPosition[0];
            depth = currentPosition[1];
            aim = currentPosition[2];
            System.out.println("Now at a depth of " + depth + " and a horizontal position of " + horizontal);
        }
        System.out.println(horizontal * depth);
        return horizontal * depth;
    }

    public static void main(String[] args) {
        plotCourse1();
        plotCourse2();
    }
}

