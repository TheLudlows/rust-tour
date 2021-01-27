import java.util.Arrays;

class Solution {
    public String minNumber(int[] nums) {
        String[] arr = new String[nums.length];
        for (int i=0;i<nums.length;i++) {
            arr[i] = String.valueOf(nums[i]);
        }
        Arrays.sort(arr, (e1, e2) -> (e1 + e2).compareTo(e2 + e1));
        StringBuilder stringBuilder = new StringBuilder();
        for (String s: arr) {
            stringBuilder.append(s);
        }

        return stringBuilder.toString();

    }
}