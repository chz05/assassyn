# Goal Refactor Python Test Wrapper of Ramualtor2

This TODO will resolve 2 issues of Python Ramulator2 Wrapper infra:
1. Similar to the problem we solved in [this TODO](../dones/DONE-wrapper-platform.md), currently [Python wrapper](../python/assassyn/ramulator2/ramulator2.md) also have a DLL loading issue, which should be fixed.
2. The [test](../python/unit-tests/test_ramulator2.py) and the [test result validator](../python/unit-tests/compare_ramulator2_outputs.py) are separated.

# Action Items

1. Fix the Python ramulator2 wrapper library load as per the modified [document](../python/assassyn/ramulator2/ramulator2.md).
2. After fixing the python loading, make sure the current [test case](../python/unit-tests/test_ramulator2.py) runs.
3. Then simplify the existing [cross validation script](../python/unit-tests/compare_ramulator2_outputs.py) as per the [reduce document](../python/unit-tests/trilang-x-valid.md).
   - Make sure it runs properly to invoke all three language of wrappers.
   - Stage and commit without verification.
4. Then combine [cross validation script](../python/unit-tests/compare_ramulator2_outputs.py) and [test case](../python/unit-tests/test_ramulator2.py) into one python file so that it can be a single pytest case.
   - Make sure this test case runs.
5. Add `pytest -n 8 -x python/unit-tests` to [pre-commit hook](../scripts/pre-commit).
6. Add `pytest -n 8 -x python/unit-tests` to [github workflow](../.github/workflows/test.yaml) right before the Python Frontend Test.
7. Stage and commit with verfication.