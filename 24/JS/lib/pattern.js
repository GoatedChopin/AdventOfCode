export const combinations = (arr, groupSize) => {
    const result = [];

    // Helper function to generate combinations recursively
    function combine(start, currentCombination) {
      // If the current combination has reached the desired group size, add it to the result
      if (currentCombination.length === groupSize) {
        result.push([...currentCombination]);
        return;
      }
  
      // Loop through the array and build combinations
      for (let i = start; i < arr.length; i++) {
        currentCombination.push(arr[i]);
        combine(i + 1, currentCombination);
        currentCombination.pop(); // Backtrack
      }
    }
  
    // Start the recursive combination generation
    combine(0, []);
  
    return result;
}

// const a = [1,2,3,4,5];
// console.log(combinations(a, 1));