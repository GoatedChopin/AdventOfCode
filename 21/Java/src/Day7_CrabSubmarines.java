import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.lang.Math;

public class Day7_CrabSubmarines {

    public static void main(String[] args) {
        System.out.println(crabPositions.stream().mapToDouble(d -> d).max());
        System.out.println(crabPositions.stream().mapToDouble(d -> d).min());
//        System.out.println(findOptimalPosition(crabPositions, true) + " requires " + fuelRequired(findOptimalPosition(crabPositions, true), crabPositions) + " fuel");

        // part two
        System.out.println(findOptimalPosition(crabPositions, false));
    }

    public static int findOptimalPosition(List<Integer> positions, boolean linear) {
        int optimalPosition = -1;
        int maxPosition = 0;
        int minPosition = 0;
        int fuelRequired;
        int optimalFuelRequired = Integer.MAX_VALUE;

        for (int i: positions) {
            if (i > maxPosition) maxPosition = i;
            if (i < minPosition) minPosition = i;
        }
        for (int i = minPosition; i <= maxPosition; i++) {
            int finalI = i;
//            fuelRequired = (int) positions.stream().mapToDouble(val -> Math.abs(finalI - val)).sum();
            if (linear == true) fuelRequired = fuelRequired(i, positions);
            else fuelRequired = fuelRequiredNonlinear(i, positions);
            System.out.println("Fuel required for position " + i + " is " + fuelRequired);

            if (fuelRequired < optimalFuelRequired) {
                optimalFuelRequired = fuelRequired;
                optimalPosition = i;
            }
        }
        System.out.println("The optimal position is " + optimalPosition + " and requires " + optimalFuelRequired + " fuel.");
        return optimalPosition;
    }

    public static int fuelRequired(int desiredPosition, List<Integer> positions) {
        int fuelRequired = -1;
//        int[] posArray = Arrays.stream(positions.toArray()).mapToInt(d -> (int) d).toArray();
//        int[] posArray = positions.stream().mapToInt(d -> (int) d).toArray();
        int[] distances = positions.stream().mapToInt(d -> Math.abs(d - desiredPosition)).toArray();
        fuelRequired = Arrays.stream(distances).sum();
        return fuelRequired;
    }

    public static int fuelRequiredNonlinear(int desiredPosition, List<Integer> positions) {
        int fuelRequired = -1;
        int max = -1;
        int min = -1;
        for (int i: positions) {
            if (i > max) max = i;
            if (i < min) min = i;
        }
        int[] fuelLookup = new int[(max - min) + 1];
        for (int i = 0; i < fuelLookup.length; i++) {
            fuelLookup[i] = fuelPerDistance(i);
        }

        fuelRequired = (int) positions.stream().mapToDouble(val -> fuelLookup[Math.abs(desiredPosition - val)]).sum();
        return fuelRequired;
    }

    private static int fuelPerDistance(int i) {
        if (i < 0) return -1;
        else if (i == 0) return i;
        else return i + fuelPerDistance(i - 1);
    }

    static File data = new File("d7data1");
    static ArrayList<Integer> crabPositions = new ArrayList<>();

    static {
        try (BufferedReader bufferedReader = new BufferedReader(new FileReader(data))) {
            String line;
            String[] split;
            while ((line = bufferedReader.readLine()) != null) {
                split = line.split(",");
                System.out.println("Adding " + split.length + " crabs");
                for (String s: split) crabPositions.add(Integer.parseInt(s.trim()));
            }

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
