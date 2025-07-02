import angr
import os
import networkx as nx
import json

def generate_call_graph(binary_path):
    """
    Generate a call graph for the given ELF binary using angr.
    """
    project = angr.Project(binary_path, auto_load_libs=False)
    cfg = project.analyses.CFGFast()

    
    call_graph = cfg.kb.callgraph

    
    call_graph_info = {}
    
    for node in call_graph.nodes():
    	func_name = cfg.kb.functions[node].name
    	call_graph_info[func_name] = {
    		"called_functions": [],
    	}
    
    for edge in call_graph.edges():
    	caller = edge[0]
    	callee = edge[1]
    	caller_name = cfg.kb.functions[caller].name
    	callee_name = cfg.kb.functions[callee].name
    	call_graph_info[caller_name]["called_functions"].append(callee_name)
    	call_graph_info[caller_name]["indegree"] = call_graph.in_degree[caller]
    	call_graph_info[caller_name]["outdegree"] = call_graph.in_degree[caller_name]
    	    

    
    return call_graph_info

def traverse_directories_and_generate_graphs(base_dir):
    """
    Traverse directories, find ELF files, and generate call graphs.
    """
    results = {}

    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith(".elf"):  
                binary_path = os.path.join(root, file)
                print(f"Processing binary: {binary_path}")
                
                
                function_info = generate_call_graph(binary_path)
                
                
                binary_name = os.path.splitext(file)[0]
                results[binary_name] = function_info
                
    return results

def save_results_to_json(results, output_dir):
    """
    Save the results dictionary to a JSON file for each binary.
    """
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    for binary_name, function_info in results.items():
        output_file = os.path.join(output_dir, f"{binary_name}.json")
        with open(output_file, 'w') as f:
            json.dump(function_info, f, indent=4)
        print(f"Saved results for {binary_name} to {output_file}")

def main(base_dir, output_dir):
    """
    Main function to orchestrate the process.
    """
    results = traverse_directories_and_generate_graphs(base_dir)
    save_results_to_json(results, output_dir)

if __name__ == "__main__":
    
    base_dir = '.'  
    output_dir = '.'  
    
    main(base_dir, output_dir)
