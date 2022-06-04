# Task scheduling simulator
疑似タスク構造を使用するスケジューリングシミュレータ

## インストール
実行ファイルを[ここ](https://github.com/dfjk0/task-scheduler-sim/releases/tag/v1.0)に置いています．
- Linux
```
$ curl -L https://github.com/dfjk0/task-scheduler-sim/releases/download/v1.0/x86_64-unknown-linux-gnu.tar.gz -o app.tar.gz
$ tar -xf app.tar.gz
$ ./app
```

- Windows
```
$ Invoke-WebRequest https://github.com/dfjk0/task-scheduler-sim/releases/download/v1.0/x86_64-pc-windows-gnu.zip -OutFile app.zip
$ Expand-Archive -Path .\app.zip
$ .\app\app.exe
```

## 使い方
### 1: Create queue list
> - タスクを格納する待ち行列を作成します．
> - 複数作成することができます．
> - それぞれの待ち行列にスケジューリングアルゴリズムを適応します．
### 2: Generate tasks
> - ランダムにタスクを生成します．
> - 先に待ち行列を作ってから実行してください．
> - タスク生成後，以下の情報をタスクごとに出力します．
> > name     : 割り当てられた名前  
> > arrive   : 到着時間  
> > cost     : 実行コスト(時間)  
> > priority : 優先順位，数値が小さいほど優先度が高い  
### 3: Run simulator
> - スケジューラをシミュレートします．
> - 当然，待ち行列とタスクが未設定だと実行できません．
> - Enterキーを押して，1ステップ進めます．
> - シミュレーションが終了すると結果を以下の情報をタスクごとに出力します．
> > name       : 割り当てられた名前  
> > arrive     : 到着時間  
> > cost       : 実行コスト(時間)  
> > priority   : 優先順位，数値が小さいほど優先度が高い  
> > finish     : 終了時刻  
> > turnaround : ターンアラウンドタイム  
> > 平均ターンアラウンドタイム
### 4: Exit
> - プログラムを終了します  

1度作成した待ち行列，タスクはシミュレーションを実行しても保持されます．これにより，同じタスクセットで複数のアルゴリズムを適応し，比較することもできます．
## 例
`>` に続く数値または文字が入力を表しています．
### 到着順でスケジューラをシミュレートしたい場合

```
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 1
Number of Queue (1~10):
> 1
Queue 0 Algorithm (1~3):
1: ArrivalOrder, 2: ProcessingTimeOrder, 3: RoundRobin
> 1
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 2
Tasks was generated.

-- Task Informations ----------------------
name arrive cost priority
   A      3    3        0
   B      2    8        0
   C      7    2        0
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 3

-- Start Simulator ------------------------
Time 0-1
Task Queue 0: [ ]
---------------------------------------
Press enter key to continue...
Time 1-2
Task Queue 0: [ ]
---------------------------------------
Press enter key to continue...
Time 2-3
    Task B arrived on Queue 0.
    Task B was dispatched and executed.
Task Queue 0: [ B ]
---------------------------------------
Press enter key to continue...
Time 3-4
    Task A arrived on Queue 0.
    Task B was executed.
Task Queue 0: [ B A ]
---------------------------------------

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  省略
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Press enter key to continue...
Time 13-14
    Task C was dispatched and executed.
Task Queue 0: [ C ]
---------------------------------------
Press enter key to continue...
Time 14-15
    Task C was executed.
    Task C was finished.
Task Queue 0: [ ]
---------------------------------------
Press enter key to continue...

-- Result ---------------------------------
name arrive cost priority finish turnaround
   B      2    8        0     10          8
   A      3    3        0     13         10
   C      7    2        0     15          8

Average of Turnaround Time: 8.666667
Press enter key to continue...
```

### 待ち行列数3，タイムクォンタム2のラウンドロビンを使い，多重レベルフィードバックスケジューリングを行いたい場合

```
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 1
Number of Queue (1~10):
> 3
Queue 0 Algorithm (1~3):
1: ArrivalOrder, 2: ProcessingTimeOrder, 3: RoundRobin
> 3
RoundRobin: Time Quantum (1~10):
> 2
RoundRobin: Multilevel Feedback? [y/n]
> y
Queue 1 Algorithm (1~3):
1: ArrivalOrder, 2: ProcessingTimeOrder, 3: RoundRobin
> 3
RoundRobin: Time Quantum (1~10):
> 2
RoundRobin: Multilevel Feedback? [y/n]
> y
Queue 2 Algorithm (1~3):
1: ArrivalOrder, 2: ProcessingTimeOrder, 3: RoundRobin
> 3
RoundRobin: Time Quantum (1~10):
> 2
RoundRobin: Multilevel Feedback? [y/n]
> y
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 2
Tasks was generated.

-- Task Informations ----------------------
name arrive cost priority
   A      0    3        1
   B      1    6        0
   C      4    1        0
   D      0    7        1
   E      3    5        1
   F      9    7        0
   G      2    8        2
-------- [Task scheduler simulator] --------
 What do you want to do?
 1: Create queue list.
 2: Generate tasks.
 3: Run simulation.
 4: Exit
> 3

-- Start Simulator ------------------------
Time 0-1
    Task A arrived on Queue 1.
    Task D arrived on Queue 1.
    Task A was dispatched and executed.
Task Queue 0: [ ]
Task Queue 1: [ A D ]
Task Queue 2: [ ]
---------------------------------------
Press enter key to continue...
Time 1-2
    Task B arrived on Queue 0.
    Task B was dispatched and executed.
Task Queue 0: [ B ]
Task Queue 1: [ A D ]
Task Queue 2: [ ]
---------------------------------------
Press enter key to continue...
Time 2-3
    Task G arrived on Queue 2.
    Task B was executed.
    Timeout Task B
Task Queue 0: [ ]
Task Queue 1: [ A D B ]
Task Queue 2: [ G ]

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  省略
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

---------------------------------------
Press enter key to continue...
Time 34-35
    Task D was dispatched and executed.
    Task D was finished.
Task Queue 0: [ ]
Task Queue 1: [ ]
Task Queue 2: [ G ]
---------------------------------------
Press enter key to continue...
Time 35-36
    Task G was dispatched and executed.
Task Queue 0: [ ]
Task Queue 1: [ ]
Task Queue 2: [ G ]
---------------------------------------
Press enter key to continue...
Time 36-37
    Task G was executed.
    Task G was finished.
Task Queue 0: [ ]
Task Queue 1: [ ]
Task Queue 2: [ ]
---------------------------------------
Press enter key to continue...

-- Result ---------------------------------
name arrive cost priority finish turnaround
   C      4    1        0      5          1
   A      0    3        1     18         18
   B      1    6        0     22         21
   E      3    5        1     31         28
   F      9    7        0     32         23
   D      0    7        1     35         35
   G      2    8        2     37         35

Average of Turnaround Time: 23
```
