#### 判断是否存在环

```java
public class Solution {
    public boolean hasCycle(ListNode head) {
        if (head == null) {
            return false;
        }
        ListNode l = head;
        ListNode r = head;
        while (r != null && r.next != null) {
            l = l.next;
            r = r.next.next;
            if (l == r) {
                return true;
            }
        }
        return false;
    }
}

// 哈希表
public class Solution {
    public boolean hasCycle(ListNode head) {
        Set<ListNode> seen = new HashSet<ListNode>();
        while (head != null) {
            if (!seen.add(head)) {
                return true;
            }
            head = head.next;
        }
        return false;
    }
}
```

#### 入环的节点

```java
public class Solution {
    public ListNode detectCycle(ListNode head) {
        Set<ListNode> seen = new HashSet<ListNode>();
        while (head != null) {
            if (!seen.add(head)) {
                return head;
            }
            head = head.next;
        }
        return null;
    }
}

// todo 快慢指针
```