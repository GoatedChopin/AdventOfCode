import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Scanner;

public class Day3_Diagnostic {
    static File dataFile = new File("C:\\Users\\Colby\\Programming\\Java\\AdventOfCode\\d3data1");
    static ArrayList<String> dataList = new ArrayList<>();
    static ArrayList<Integer> gamma = new ArrayList<>();
    static ArrayList<Integer> epsilon = new ArrayList<>();

    static {
        try (Scanner scanner = new Scanner(new FileReader(dataFile))) {
            while (scanner.hasNext()) {
                dataList.add(scanner.nextLine());
            }
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
    }

    public static int findModeOfPosition(int bitPosition, ArrayList<String> dataList) {
        int zeroCount = 0;
        int oneCount = 0;
        for (int i = 0; i < dataList.size(); i++) {
            if (dataList.get(i).charAt(bitPosition) == '0') zeroCount++;
            else oneCount++;
        }
        System.out.println("There were " + zeroCount +" zeros and " + oneCount + " ones");

        if (zeroCount > oneCount) {
            gamma.add(0);
            epsilon.add(1);
            return 0;
        }
        else {
            gamma.add(1);
            epsilon.add(0);
            return 1;
        }
    }

    public static int convertListToDecimal(ArrayList<Integer> arrayList) {
        String outputString = arrayList.get(0).toString();
        for (int i = 1; i < arrayList.size(); i++) {
            outputString = outputString.concat(arrayList.get(i).toString());
        }
        System.out.println(outputString);
        return Integer.parseInt(outputString, 2);
    }

    public static ArrayList<String> filterFindRating(int bitPosition, ArrayList<String> arrayList, boolean useMax) {
        String mode = Integer.toString(findModeOfPosition2(bitPosition, arrayList));
        ArrayList<Integer> removalList = new ArrayList();
        if (useMax) {
            for (int i = 0; i < arrayList.size(); i++) if (!arrayList.get(i).substring(bitPosition, bitPosition + 1).equals(mode)) removalList.add(i);
        } else {
            for (int w = 0; w < arrayList.size(); w++) if (arrayList.get(w).substring(bitPosition,bitPosition+1).equals(mode)) removalList.add(w);
        }
        arrayList.removeAll(removalList);
        return arrayList;
    }

    public static int findModeOfPosition2(int bitPosition, ArrayList<String> dataList) {
        int zeroCount = 0;
        int oneCount = 0;
        for (int i = 0; i < dataList.size(); i++) {
            if (dataList.get(i).charAt(bitPosition) == '0') zeroCount++;
            else oneCount++;
        }
        System.out.println("There were " + zeroCount +" zeros and " + oneCount + " ones");

        if (zeroCount > oneCount) {
            return 0;
        }
        else {
            return 1;
        }
    }

    public static void main(String[] args) {
        for (int i = 0; i < dataList.get(0).length(); i++) {
            findModeOfPosition(i, dataList);
        }
        System.out.println(gamma);
        System.out.println(epsilon);
        int gammaInt = convertListToDecimal(gamma);
        int epsilonInt = convertListToDecimal(epsilon);
        System.out.println(gammaInt * epsilonInt);

        for (int i = 0; i < dataList.get(0).length(); i++) {
            if (dataList.size() > 1) {
                dataList = filterFindRating(i, dataList, true);
            }
        }
        String oxygenGeneratorRating = dataList.get(0);
        System.out.println("Oxygen Generator Rating = " + oxygenGeneratorRating);
        System.out.println(Integer.parseInt(oxygenGeneratorRating, 2));
        dataList.clear();
        try (Scanner scanner = new Scanner(new FileReader(dataFile))) {
            while (scanner.hasNext()) {
                dataList.add(scanner.nextLine());
            }
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        for (int i = 0; i < dataList.get(0).length(); i++) {
            if (dataList.size() > 1) {
                dataList = filterFindRating(i, dataList, false);
            }
        }
        String co2ScrubberRating = dataList.get(0);
        System.out.println("CO2 Scrubber Rating = " + co2ScrubberRating);
        System.out.println(Integer.parseInt(co2ScrubberRating, 2));
        // Answer Incorrect, needs revision
        System.out.println(Integer.parseInt(co2ScrubberRating, 2) * Integer.parseInt(oxygenGeneratorRating, 2));
    }
}