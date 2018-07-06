#[no_mangle]
pub extern "C" fn resolve_module() {
    println!("Hello from rust resolve_module");
    /*
	moduleName, filename, err := ResolveModule(moduleSpecifier, containingFile)
	if err != nil {
		return
	}

	logDebug("CodeFetch moduleName %s moduleSpecifier %s containingFile %s filename %s",
		moduleName, moduleSpecifier, containingFile, filename)

	if isRemote(moduleName) {
		sourceCodeBuf, err = FetchRemoteSource(moduleName, filename)
	} else if strings.HasPrefix(moduleName, assetPrefix) {
		f := strings.TrimPrefix(moduleName, assetPrefix)
		sourceCodeBuf, err = Asset(path.Join("dist", f))
		if err != nil {
			logDebug("%s Asset doesn't exist. Return without error", moduleName)
			err = nil
		}
	} else {
		assert(moduleName == filename,
			"if a module isn't remote, it should have the same filename")
		sourceCodeBuf, err = ioutil.ReadFile(moduleName)
	}
	if err != nil {
		return
	}

	outputCode, err := LoadOutputCodeCache(filename, sourceCodeBuf)
	if err != nil {
		return
	}

	var sourceCode = string(sourceCodeBuf)
	res = &Msg{
		Command:                Msg_CODE_FETCH_RES,
		CodeFetchResModuleName: moduleName,
		CodeFetchResFilename:   filename,
		CodeFetchResSourceCode: sourceCode,
		CodeFetchResOutputCode: outputCode,
	}
*/
}
