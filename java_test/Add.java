class Sub {
  static int sub(int a, int b) {
    return a - b;
  }
}

class LTheNumber1 {
  int localvalue;
  static int value = 1;

  void aaaa(int num) {
    localvalue = num;
  }
}

class Add {
  static int add(int a, int b) {
    return a + b;
  }

  public static void main(String[] args) {
    var a = add(5,6);
    var sub = new Sub();
    var b = sub.sub(10,5);
    int c = LTheNumber1.value;
    var d = new LTheNumber1();
    d.aaaa(10);
  }

  void aaa(LTheNumber1 a) {
    a.aaaa(69);
  }
}
