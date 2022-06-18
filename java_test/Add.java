class Sub {
  static int sub(int a, int b) {
    return a - b;
  }
}

class TheNumber1 {
  static int value = 1;
}

class Add {
  static int add(int a, int b) {
    return a + b;
  }

  public static void main(String[] args) {
    var a = add(5,6);
    var sub = new Sub();
    var b = sub.sub(10,5);
    int c = TheNumber1.value;
  }
}
