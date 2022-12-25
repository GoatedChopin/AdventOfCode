import java.io.*;
import java.util.ArrayList;
import java.lang.Math;

public class Day5_ThermalVents {
    static File data = new File("C:\\Users\\colby\\IdeaProjects\\AdventOfCode21\\d5data1");
    static String line;
    static int[][] ventMap;
    static int maxMapSize = 1;
    static ArrayList<Integer[]> coordinates = new ArrayList<>();
    static int crossings;

    static {
        try(BufferedReader bufferedReader = new BufferedReader(new FileReader(data))) {
            while((line = bufferedReader.readLine()) != null) {
                String[] lineArray = line.split("->");
                String[] vector1 = lineArray[0].split(",");
                String[] vector2 = lineArray[1].split(",");

                int x1 = Integer.parseInt(vector1[0].trim());
                int y1 = Integer.parseInt(vector1[1].trim());
                int x2 = Integer.parseInt(vector2[0].trim());
                int y2 = Integer.parseInt(vector2[1].trim());
                Integer[] coords = {x1, y1, x2, y2};
                coordinates.add(coords);

                for(int i: coords) {
                    if(i > maxMapSize) maxMapSize = i;
                }
            }

            ventMap = new int[maxMapSize][maxMapSize];
            for(Integer[] coordArray: coordinates) {
                int xdif = coordArray[0] - coordArray[2];
                int ydif = coordArray[1] - coordArray[3];

                if (xdif == 0) {
                    for (int i = Math.min(coordArray[1], coordArray[3]); i < Math.max(coordArray[1], coordArray[3]); i++) {
                        ventMap[coordArray[0]][i]++;
//                        if(ventMap[coordArray[0]][i] == 2) crossings++;
                    }
                }
                if (ydif == 0) {
                    for (int i = Math.min(coordArray[0], coordArray[2]); i < Math.max(coordArray[0], coordArray[2]); i++) {
                        ventMap[i][coordArray[1]]++;
//                        if(ventMap[i][coordArray[1]] == 2) crossings++;
                    }
                }
            }
            for(int[] intArray: ventMap) {
                for(int i: intArray) {
                    if(i > 1) {
                        crossings++;
                    }
                }
            }

        } catch(IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        System.out.println(crossings);
        System.out.println(ventMap.toString());
    }
}
