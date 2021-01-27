
class Solution {
    public boolean exist(char[][] board, String word) {

        boolean[][] vis = new boolean[board.length][board[0].length];
        char[] chars = word.toCharArray();
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[0].length; j++) {
                if (find(board, i, j, 0, chars, vis)) {
                    return true;
                }
            }
        }
        return false;
    }

    public boolean find(char[][] board, int x, int y, int i, char[] chars, boolean[][] vis) {
        if (x < 0 || y < 0 || x >= board.length || y >= board[0].length || vis[x][y] || chars[i] != board[x][y]) {
            return false;
        }
        if (i == chars.length - 1) {
            return true;
        }
        vis[x][y] = true;
        boolean find = find(board, x, y + 1, i + 1, chars, vis) ||
                find(board, x - 1, y, i + 1, chars, vis) ||
                find(board, x, y - 1, i + 1, chars, vis) ||
                find(board, x + 1, y, i + 1, chars, vis);
        vis[x][y] = false;
        return find;
    }

    public static void main(String[] args) {

    }
}